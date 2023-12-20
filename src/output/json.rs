//! This module contains implementation details for
//! StreamPublisherConnection when `JSON` feature is
//! enabeld.  This allows StreamPublisherConnection to
//! publish to json files in a directory.
//!
//! //! This module contains implementation deatils for
//! StreamPublisherConnection when `JSON` feature is
//! enabled.  This allows StreamPublisherConnection
//! to publish to a local JSONL file

use prost::Message;
use serde::Serialize;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use super::publish::{StreamPublisherConnection, StreamPublisherConnectionClient};

/// Opens the connection to a JSONL file.
pub async fn connect(queue_env: &str) -> StreamPublisherConnection {
    // Get expected output directory as a string
    let output_dir_string = dotenvy::var("OUTPUT_DIR")
        .expect("OUTPUT_DIR should exist in .env file")
        .parse::<String>()
        .unwrap();

    // transform it into a path object
    let mut output_dir = PathBuf::new();
    output_dir.push(output_dir_string);
    let subdirectory = dotenvy::var(queue_env)
        .unwrap_or_else(|_| panic!("{} should exist in the .env file", queue_env));
    output_dir.push(subdirectory.clone());
    // transform it into a path object
    create_dir_all(&output_dir).expect("directory creation permissions and storage available");

    // Return the created connection
    StreamPublisherConnection {
        client: StreamPublisherConnectionClient::Json(output_dir),
        queue_name: subdirectory.to_string(),
    }
}

impl StreamPublisherConnectionClient {
    /// Publish a prost message to a json file with the given name
    #[inline]
    pub async fn publish<T: Serialize + Message>(&self, name: &str, msg: T) {
        // Extract the jsonfile_arc
        let StreamPublisherConnectionClient::Json(directory) = self;
        // Create an example filepath
        let mut filepath = directory.join(String::from(name) + ".json");
        // Recreate the filepath
        while filepath.exists() {
            filepath = directory.join(String::from(name) + ".json");
        }
        // Create and write to the file
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(filepath)
            .expect("Failed to open file");
        let json = serde_json::to_string::<T>(&msg).unwrap();
        writeln!(file, "{}", json).expect("storage is writable");
    }
}

impl StreamPublisherConnection {
    /// Publish a prost message to a JSON file with the given name
    #[inline]
    pub async fn publish<T: Serialize + Message>(&self, name: &str, msg: T) {
        self.client.publish(name, msg).await;
    }
}
