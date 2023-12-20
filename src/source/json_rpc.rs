use super::config::RequestConfig;
use crate::{constants, metrics::Metrics, request};
use log::{error, info, warn};
use std::time;
use tokio::time::{sleep, timeout, Duration};

#[cfg(feature = "SOLANA")]
use crate::solana_config::{
    data_sources::json_rpc,
    types::{block_response_types, blockheight_response_types, slot_response_types},
};

/// Creates a new request builder using the fallback endpoint.
pub async fn use_fallback_endpoint(request: String, endpoint: String) -> reqwest::RequestBuilder {
    let fallback_client = reqwest::Client::new();
    let headers = request::get_headers();
    let fallback_endpoint = endpoint.parse::<String>().unwrap();
    fallback_client
        .post(fallback_endpoint)
        .body(request.clone())
        .headers(headers)
}

/// calls the rpc method in a loop with delay to account for potential disconnections.
/// `RPC_METHOD_TIMEOUT` can be added to the .env file to adjust the timeout, otherwise it is 5 seconds.
/// `RPC_FALLBACK_THRESHOLD` can be added to the .env file to adjust how many attempts until we try
///     to utilize the fallback server.
#[allow(non_snake_case)]
pub async fn call_rpc_method(
    request_config: RequestConfig,
    request: String,
    metrics: Option<Metrics>,
) -> reqwest::Response {
    // Get the fallback threshold (how many attempts until we use fallback)
    let fallback_threshold = dotenvy::var("RPC_FALLBACK_THRESHOLD")
        .unwrap_or_else(|_| "2".to_string())
        .parse()
        .unwrap_or(2);

    let response_timeout = Duration::from_secs(constants::RESPONSE_TIMEOUT);

    let request_builder = request_config.to_requestbuilder();

    // Create the request builder
    let mut cur_request_builder = request_builder.try_clone().unwrap().body(request.clone());

    // Attempt to connect to the server
    let mut i = 1;
    loop {
        // after enough failures, switch to the fallback endpoint, if there is one in the environment variables.
        if i == fallback_threshold {
            if let Ok(fb) = dotenvy::var("FALLBACK_ENDPOINT") {
                info!("Switching to the fallback endpoint");
                cur_request_builder = use_fallback_endpoint(request.clone(), fb).await;
            }
        }

        // Update request count metrics
        if let Some(m) = &metrics {
            m.request_count.inc();
        }

        let cur_request = cur_request_builder.try_clone().unwrap().send();
        let response_timer = timeout(response_timeout, cur_request);
        match response_timer.await {
            Err(_) => warn!("Request timed out. Re-attempting..."),
            Ok(response) => match response {
                Err(e) => {
                    error!("Request failed (attempt #{}): {:?}", i, e);
                    let seconds = time::Duration::from_secs(2);
                    sleep(seconds).await;
                }
                Ok(r) => {
                    info!(
                        "Request for {} succeeded ({} attempt(s))",
                        request.clone(),
                        i
                    );
                    return r;
                }
            },
        }

        // Update failed request metrics

        if let Some(m) = &metrics {
            m.request_count.inc();
        }

        i += 1;
    }
}

#[cfg(feature = "SOLANA")]
#[allow(non_snake_case)]
pub async fn call_getSlot(request_config: RequestConfig, metrics: Option<Metrics>) -> u64 {
    info!("making a request for getslot");
    loop {
        let response: reqwest::Response = call_rpc_method(
            request_config.try_clone().unwrap(),
            json_rpc::get_slot_post_body(),
            metrics.clone(),
        )
        .await;
        info!("Deserializing a response for getslot");
        // deserialize the response
        match response.json::<slot_response_types::SlotResponse>().await {
            Ok(val) => return val.get_slot(),
            Err(_) => {
                warn!("Could not parse the response for getSlot(). Re-requesting...");
                continue;
            }
        }
    }
}
/// the getBlockHeight() call is always the same. so, we create one at startup, then clone it each time we need it.
#[allow(non_snake_case)]
pub async fn call_getBlockHeight(request_config: RequestConfig, metrics: Option<Metrics>) -> u64 {
    let getBlockHeight_request = json_rpc::get_block_height_post_body();
    info!("making a request for getblockheight");
    let response: reqwest::Response =
        call_rpc_method(request_config, getBlockHeight_request, metrics).await;
    info!("Deserializing a response for getblockheight");
    // deserialize the response
    response
        .json::<blockheight_response_types::BlockHeightResponse>()
        .await
        .expect("FATAL: failed to parse the response for getBlockHeight()")
        .get_block_height()
}

/// makes a request for the block at the index, and returns the deserialized response.
#[allow(non_snake_case)]
pub async fn call_getBlock(
    request_config: RequestConfig,
    block_index: u64,
    metrics: Option<Metrics>,
) -> block_response_types::BlockResponse {
    loop {
        let getBlock_request = json_rpc::get_block_post_body(block_index);
        info!("making a request for getblock");
        let response: reqwest::Response = call_rpc_method(
            request_config.try_clone().unwrap(),
            getBlock_request,
            metrics.clone(),
        )
        .await;
        /*
            Uses the simd_json library to accelerate response deserialization in Release profile.
            Uses the serde_json library in debug profile, because it provides more useful error messages,
                such as the position of the error in the response, as well as the value.

            NOTE: if something is broken in the simd_json crate, then we will not notice it if we compile in debug mode.
        */
        #[cfg(debug_assertions)]
        {
            let r = &mut response
                .text()
                .await
                .expect("FATAL: could not parse the response as a string");
            //debug!("Block response text is {}", r.clone());
            if r.contains("Too many requests for a specific RPC call, contact your app developer or support@rpcpool.com.") {
                error!("throttled by the public node");
                let seconds = time::Duration::from_secs(3);
                sleep(seconds).await;
            } else {
                // the `serde_json` deserializer produces more information for debugging (such as the column in the string, and the value)
                //  but it is slower than `simd_json`.
                let Block_response: block_response_types::BlockResponse = serde_json::from_str(r).unwrap();
                if let Some(err) = Block_response.error.as_ref() {
                    warn!("block response error: {:?}", err);
                } else {
                    info!("Successfully deserialized a block at index {}", block_index);
                }
	        return Block_response;
            }
        }
        #[cfg(not(debug_assertions))]
        {
            // Deserialize the block response using the simd_json library to take advantage of CPU vector extensions (e.g. SSE, AVX, NEON)
            let deserialized_response: Result<
                block_response_types::BlockResponse,
                simd_json::Error,
            > = {
                let response_bytes = match response.bytes().await {
                    Err(e) => {
                        error!("could not read response: {:?}", e);
                        continue;
                    }
                    Ok(val) => val,
                };
                let mut byte_vec = response_bytes.to_vec();
                let byte_slice = byte_vec.as_mut_slice();
                simd_json::from_slice(byte_slice)
            };

            match deserialized_response {
                Ok(block_response) => return block_response,
                Err(deserialization_err) => {
                    warn!(
                        "Failed to deserialize the block response for index {}. Error code: {:?}",
                        block_index, deserialization_err
                    );
                    continue;
                }
            }
        }
    }
}

#[allow(non_snake_case)]
pub async fn call_getBlocks(
    request_config: RequestConfig,
    start: u64,
    end: u64,
    metrics: Option<Metrics>,
) -> Vec<u64> {
    let getBlocks_request = json_rpc::get_blocks_post_body(start, end);
    info!("making a request for getblocks");
    let response: reqwest::Response =
        call_rpc_method(request_config, getBlocks_request, metrics).await;
    info!("Deserializing a response for getblocks");
    // deserialize the response
    response
        .json::<block_response_types::BlocksResponse>()
        .await
        .expect("FATAL: failed to parse the response for getBlockHeight()")
        .result
}
