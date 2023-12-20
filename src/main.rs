// Note:  Cargo automated documentation doesn't apply to the main.rs file.  To write documentation
// for the index of the wiki page, do so in the lib.rs file.
use actix_web::{get, App, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use clap::{Args, Parser, Subcommand};
use log::info;
use std::error::Error;
use std::fs::{create_dir, read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

mod benchmark;
pub mod constants;
mod request;
use blockchain_etl_indexer::metrics::Metrics;

// Get the config associated with the chosen blockchain.  We should import the config as
// `blockchain_config` so we can use the blockchain configuration generically.
#[cfg(feature = "SOLANA")]
use blockchain_etl_indexer::solana_config::lib as blockchain_config;

#[cfg(feature = "SOLANA_BIGTABLE")]
use blockchain_etl_indexer::solana_config::data_sources::bigtable;

// On platforms other than Windows, import the jemallocator library.
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

/// On platforms other than Windows, we use Jemalloc as the global allocator
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// CLI Parsing information
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Time (in minutes) to measure the blockchain network's throughput
    Benchmark { time: u64 },
    /// Extract blocks from a starting index
    IndexRange(IndexRangeArgs),
    /// Extract blocks from a list
    IndexList(IndexListArgs),
}

/// Arguments relating the the indexing of the crypto currency, particularly output,
/// start point, and direction (reverse)
#[derive(Args)]
struct IndexRangeArgs {
    /// OutputType is the object expected to be used to send the data extracted by the Indexer.
    /// Currently only supporting streaming to a message-passing queue, however in the future we
    /// may support output to CSV files, json files, or parquet.
    out: OutputType,
    /// The slot to begin indexing from
    start: u64,
    /// The slot to stop indexing at
    end: Option<u64>,
    /// Index backwards towards the genesis block
    #[clap(long)] // Long flag format ('--reverse')
    reverse: bool,
}

/// Arguments relating the the indexing of the crypto currency, particularly output,
/// start point, and direction (reverse)
#[derive(Args)]
struct IndexListArgs {
    /// OutputType is the object expected to be used to send the data extracted by the Indexer.
    /// Currently only supporting streaming to a message-passing queue, however in the future we
    /// may support output to CSV files, json files, or parquet.
    out: OutputType,
    /// The path to a list of blocks to index.
    list: String,
}

/// The possible output types for the extracted data
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum OutputType {
    /// Stream the data to a queue (e.g. google pub/sub)
    Stream,
}

/// Returns Welcome message when accessing the base-url of the server
#[get("/")]
async fn index() -> impl Responder {
    "Welcome to ETL Metrics Server."
}

/// Reads in a CSV of u64 values and returns an iterator over the values.
fn read_block_list_csv(file_path: &Path) -> Box<dyn Iterator<Item = u64>> {
    // determine if the first line of the csv seems like a header
    let has_headers = {
        let file = File::open(file_path).expect("file exists");
        let mut buf_reader = BufReader::new(file);
        let mut first_line = String::new();
        buf_reader
            .read_line(&mut first_line)
            .expect("file is readable");
        first_line
            .split(',')
            .all(|field| field.parse::<u64>().is_err())
    };

    // create the csv reader with the apparent header setting
    let rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(file_path)
        .expect("csv exists and is readable");

    // read the csv as string values
    let raw_records: Vec<csv::StringRecord> = rdr
        .into_records()
        .map(|rec_result| rec_result.unwrap())
        .collect();

    // convert the strings to integers
    let parsed_records: Vec<Vec<u64>> = raw_records
        .into_iter()
        .map(|record| {
            record
                .into_iter()
                .map(|val| val.parse::<u64>().expect("value in record is a u64"))
                .collect::<Vec<u64>>()
        })
        .collect();

    // flatten all records into a single vector
    let values: Vec<u64> = parsed_records.into_iter().flatten().collect();

    // return an iterator over the values
    let values_iter = values.into_iter();

    Box::new(values_iter)
}

/// Opens the directory of indexed block numbers and determine where to pick up from.
fn pick_up_from_previous_range(
    start: u64,
    end: Option<u64>,
    is_reverse: bool,
) -> (u64, Option<u64>) {
    let mut range_start = start;
    let mut range_end = end;
    // check for the index logs to pick up from a terminated run.
    //      - create the directory if it doesn't exist.
    let indexed_blocks_dir = Path::new("./indexed_blocks/");
    if indexed_blocks_dir.exists() {
        let indexed_blocks_files = read_dir(indexed_blocks_dir).unwrap();
        // the order that the files are iterated through is platform-dependent,
        // so we treat it as an unsorted list.
        for file in indexed_blocks_files {
            let path = file.unwrap().path();
            let filename = path.file_name().unwrap().to_str().unwrap();

            if is_reverse {
                let previous_start: u64 = filename.parse().expect("file name is a u64");
                match range_end {
                    Some(e) => {
                        if previous_start <= e {
                            range_end = Some(previous_start - 1);
                        }
                    }
                    None => {
                        if previous_start <= range_start {
                            range_start = previous_start - 1;
                        }
                    }
                }
            } else {
                let previous_end: u64 = filename.parse().expect("file name is a u64");
                if previous_end >= range_start {
                    range_start = previous_end + 1;
                    info!("setting range_start to {range_start}");
                }

                // ensure that we don't continue indexing beyond the range end, if passed in.
                if let Some(e) = end {
                    if previous_end >= e {
                        panic!("This range has already been indexed. Stopping...");
                    }
                }
            }
        }
    } else {
        create_dir(indexed_blocks_dir).expect("filesystem is writable");
    }

    (range_start, range_end)
}

/// Main function for the ETL-Core code.  Performs the following startup-tasks:
/// - Setup the logging system
/// - Loads in the .env
/// - Set up the RequestBuilder (reuse the same client)
/// - Setup the Prometheus metrics system
/// - Setup the stream connection (whether it is pubsub, rabbitmq, etc)
/// The main function then proceeds to run the indexer (either the custom indexer or the crypto-specific extract_all)
#[tokio::main]
#[allow(non_snake_case)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the logger
    env_logger::init();

    // Loads the .env file, raises an error otherwise
    dotenvy::dotenv().expect(".env file is required");

    // Set up the RequestBuilder to be used in the ETL-Core code.
    // NOTE: the reqwest docs suggest reusing a single client, rather than using multiple
    // the endpoint and request headers will be the same for every request, so
    // we will clone this request builder, rather than constructing a new one every time.
    let request_builder = {
        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();
        let connection_timeout = std::time::Duration::from_secs(constants::CONNECTION_TIMEOUT);
        let client_builder = reqwest::Client::builder().connect_timeout(connection_timeout);

        let client = client_builder.build().unwrap();
        let headers = request::get_headers();
        client.post(endpoint).headers(headers)
    };

    let cli = Cli::parse();

    // env:num_extractor_threads = The number of threads to use for extracting data.
    let num_extractor_threads = dotenvy::var("NUM_EXTRACTOR_THREADS")
        .expect("NUM_EXTRACTOR_THREADS should exist in .env file")
        .parse::<usize>()
        .unwrap();

    // env:enable_metrics = Whether we should be tracking metrics (prometheus connection)
    let enable_metrics = dotenvy::var("ENABLE_METRICS")
        .expect("ENABLE_METRICS should exist in .env file")
        .parse::<bool>()
        .unwrap();

    // metrics setup
    // - Reads in the metrics address and port from the .env file
    // - Sets up th prometheus metrics server
    let (metrics, srv_handle) = if enable_metrics {
        let (metrics_address, metrics_port) = {
            // env:metrics_address = Address for connecting to the Prometheus server
            let metrics_address = dotenvy::var("METRICS_ADDRESS")
                .expect("METRICS_ADDRESS should exist in .env file")
                .parse::<String>()
                .unwrap();
            // env:metrics_port = Port for connecting to the Prometheus server
            let metrics_port = dotenvy::var("METRICS_PORT")
                .expect("METRICS_PORT should exist in .env file")
                .parse::<u16>()
                .unwrap();
            (metrics_address, metrics_port)
        };

        let prometheus = PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .build()
            .unwrap();

        let request_count =
            prometheus::IntCounter::new("request_count", "Total number of requests for all APIs")
                .unwrap();
        let failed_request_count = prometheus::IntCounter::new(
            "failed_request_count",
            "Total number of request failures for all APIs",
        )
        .unwrap();
        prometheus
            .registry
            .register(Box::new(request_count.clone()))
            .unwrap();
        prometheus
            .registry
            .register(Box::new(failed_request_count.clone()))
            .unwrap();

        let srv = HttpServer::new(move || App::new().wrap(prometheus.clone()).service(index))
            .bind((metrics_address, metrics_port))?
            .run();

        let srv_handle = srv.handle();

        tokio::task::spawn(srv);

        let metrics = Metrics {
            request_count,
            failed_request_count,
        };
        (Some(metrics), Some(srv_handle))
    } else {
        (None, None)
    };

    match cli.command {
        Commands::Benchmark { time } => {
            info!("Benchmarking blockchain network for {} minutes...", time);
            let throughput = benchmark::get_blockchain_throughput(request_builder, 1).await;
            info!("Throughput in bytes per second: {}", throughput);
            return Ok(());
        }
        Commands::IndexRange(args) => {
            if args.start == 0 && args.end.is_none() && args.reverse {
                panic!("FATAL: cannot index backwards from genesis");
            }
            let (start, opt_end) = pick_up_from_previous_range(args.start, args.end, args.reverse);

            #[allow(clippy::collapsible_else_if)]
            let indexing_range: Box<dyn Iterator<Item = u64>> = if args.reverse {
                if let Some(end) = opt_end {
                    Box::new((start..end).rev())
                } else {
                    Box::new((0..start).rev())
                }
            } else {
                if let Some(end) = opt_end {
                    Box::new(start..end)
                } else {
                    Box::new(start..)
                }
            };

            match args.out {
                OutputType::Stream => {
                    let publisher =
                        blockchain_etl_indexer::output::publish::StreamPublisher::new().await;

                    let cur_publisher = publisher.clone();
                    #[cfg(not(feature = "SOLANA_BIGTABLE"))]
                    let bigtable = None;
                    #[cfg(feature = "SOLANA_BIGTABLE")]
                    let bigtable = {
                        let _bigtable = bigtable::connect_to_bigtable().await;
                        match _bigtable {
                            Ok(bt) => Some(bt),
                            Err(e) => panic!("Failed to connect to bigtable {:?}", e),
                        }
                    };
                    blockchain_config::extract(
                        indexing_range,
                        request_builder,
                        bigtable,
                        num_extractor_threads,
                        cur_publisher,
                        metrics,
                    )
                    .await
                    .unwrap();

                    #[cfg(not(any(feature = "JSON", feature = "JSONL")))]
                    publisher.disconnect().await;
                }
            }
        }
        Commands::IndexList(args) => {
            let list_arg = args.list;
            let list_path = Path::new(&list_arg);
            let indexing_list = if list_path.exists() {
                // read the slots in from the csv file at the path
                read_block_list_csv(list_path)
            } else {
                panic!("Please pass a valid file path for the block list.");
            };

            //let debug_list: Vec<u64> = indexing_list.collect();
            //dbg!(debug_list);

            match args.out {
                OutputType::Stream => {
                    let publisher =
                        blockchain_etl_indexer::output::publish::StreamPublisher::new().await;
                    let cur_publisher = publisher.clone();
                    #[cfg(not(feature = "SOLANA_BIGTABLE"))]
                    let bigtable = None;
                    #[cfg(feature = "SOLANA_BIGTABLE")]
                    let bigtable = {
                        let _bigtable = bigtable::connect_to_bigtable().await;
                        match _bigtable {
                            Ok(bt) => Some(bt),
                            Err(e) => panic!("Failed to connect to bigtable {:?}", e),
                        }
                    };

                    blockchain_config::extract(
                        indexing_list,
                        request_builder,
                        bigtable,
                        num_extractor_threads,
                        cur_publisher,
                        metrics,
                    )
                    .await
                    .unwrap();

                    #[cfg(not(any(feature = "JSON", feature = "JSONL")))]
                    publisher.disconnect().await;
                }
            }
        }
    }

    if enable_metrics {
        srv_handle.unwrap().stop(false).await;
    }

    Ok(())
}
