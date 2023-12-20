#[cfg(test)]
mod tests {
    use crate::{
        call_getBlock, call_getBlockHeight, call_getSlot,
        solana_config::{
            accounts::{call_getMultipleAccounts, TimestampedAccounts},
            types::block_response_types,
        },
        source::config::RequestConfig,
    };
    use std::{thread, time};

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_call_getBlockHeight() {
        let http_request_client = reqwest::Client::new();

        let headers = {
            let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers
        };

        dotenvy::dotenv().expect(".env file is required");

        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();

        let request_builder = http_request_client.post(endpoint).headers(headers);
        let request_config = RequestConfig::ReqBldr(request_builder);

        let slot_val = call_getBlockHeight(request_config.try_clone().unwrap(), None).await;
        println!("Slot is {}", slot_val);
        return;
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_call_getSlot() {
        let http_request_client = reqwest::Client::new();

        let headers = {
            let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers
        };

        dotenvy::dotenv().expect(".env file is required");

        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();

        let request_builder = http_request_client.post(endpoint).headers(headers);
        let request_config = RequestConfig::ReqBldr(request_builder);

        let slot_val = call_getSlot(request_config.try_clone().unwrap(), None).await;
        println!("Slot is {}", slot_val);
        return;
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_call_getBlock_genesis() {
        let http_request_client = reqwest::Client::new();

        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers
        };

        dotenvy::dotenv().expect(".env file is required");

        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();

        let request_builder = http_request_client.post(endpoint).headers(headers);
        let request_config = RequestConfig::ReqBldr(request_builder);
        println!("making a request for the genesis block...");

        let block_response: block_response_types::BlockResponse =
            call_getBlock(request_config, 0, None).await;
        dbg!(block_response);
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_call_getBlock_5() {
        let http_request_client = reqwest::Client::new();

        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers
        };

        dotenvy::dotenv().expect(".env file is required");

        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();

        let request_builder = http_request_client.post(endpoint).headers(headers);
        let request_config = RequestConfig::ReqBldr(request_builder);

        let five_blocks: Vec<u64> = Vec::from([1, 100, 100_000, 100_000_000, 200_000_000]);
        println!("making 5 requests for known blocks...");
        for block_i in five_blocks {
            println!("making request #{}", block_i);
            println!("making a test request for block {}...", block_i);
            let block_response: block_response_types::BlockResponse =
                call_getBlock(request_config.try_clone().unwrap(), block_i, None).await;
            dbg!(block_response);
            let seconds = time::Duration::from_secs(5);
            thread::sleep(seconds);
        }
    }

    #[tokio::test]
    #[allow(non_snake_case)]
    async fn test_call_getMultipleAccounts() {
        let http_request_client = reqwest::Client::new();

        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers
        };

        dotenvy::dotenv().expect(".env file is required");

        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();

        let request_builder = http_request_client.post(endpoint).headers(headers);
        let request_config = RequestConfig::ReqBldr(request_builder);

        println!("making a request for the accounts...");

        let account_response: Option<TimestampedAccounts> = call_getMultipleAccounts(
            request_config,
            vec![
                String::from("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg"),
                String::from("4fYNw3dojWmQ4dXtSGE9epjRGy9pFSx62YypT7avPYvA"),
                String::from("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5"),
            ],
            None,
        )
        .await;
        dbg!(account_response.unwrap());
    }
}
