//! This module contains implementation deatils for
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
        client: StreamPublisherConnectionClient::JsonL(output_dir),
        queue_name: subdirectory.to_string(),
    }
}

impl StreamPublisherConnectionClient {
    /// Publish a prost message to the JSON file
    #[inline]
    pub async fn publish_batch<T: Serialize + Message>(&self, filename: &str, msg_batch: Vec<T>) {
        if msg_batch.is_empty() {
            return;
        }

        let StreamPublisherConnectionClient::JsonL(directory) = self;
        // Create an example filepath
        let mut filepath = directory.join(String::from(filename) + ".jsonl");
        // Recreate the filepath
        while filepath.exists() {
            filepath = directory.join(String::from(filename) + ".jsonl");
        }

        // Create and append to the file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .expect("Failed to open file");

        for record in msg_batch.into_iter() {
            let json = serde_json::to_string::<T>(&record).unwrap();
            writeln!(file, "{}", json).expect("storage is writable");
        }
    }

    /// Publish a prost message to the JSON file
    #[inline]
    pub async fn publish<T: Serialize + Message>(&self, name: &str, msg: T) {
        let StreamPublisherConnectionClient::JsonL(directory) = self;
        // Create an example filepath
        let mut filepath = directory.join(String::from(name) + ".json");
        // Recreate the filepath
        while filepath.exists() {
            filepath = directory.join(String::from(name) + ".json");
        }

        // Create and append to the file
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
    /// Publish a prost message to the JSON file
    #[inline]
    pub async fn publish_batch<T: Serialize + Message>(&self, filename: &str, msg_batch: Vec<T>) {
        self.client.publish_batch(filename, msg_batch).await;
    }

    /// Publish a prost message to the JSON file
    #[inline]
    pub async fn publish<T: Serialize + Message>(&self, filename: &str, msg: T) {
        self.client.publish(filename, msg).await;
    }
}
