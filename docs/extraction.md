# Data Extraction

All RPC requests are retried with backoff upon failure, with failures logged at the `warning` RUST_LOG level.

Blocks are requested from the node by the `call_getBlock()` function.

The `call_getSlot()` function requests the current highest available slot from the data source.

The `call_getMultipleAccounts()` function requests account data for a list of pubkeys. These pubkeys come from the created accounts and token mints in the block data.

The HTTP requests that these functions make are defined in `src/solana_config/types/request_types.rs` file.

# Concurrency

The master thread continually sends slot values to a concurrent queue for worker threads to index.

Long-lived threads are created at the start of runtime by the master thread, and continually pull tasks (slot values) from the concurrent queue. Each thread makes requests to the node for the block data at that slot, then deserializes the response, and transmits the data to a stream queue.
* For communication with the stream queue (which supports concurrent producers), each thread serializes its data using the protocol buffers interface, and transmits the information.
