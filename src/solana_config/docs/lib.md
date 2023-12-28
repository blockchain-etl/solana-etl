# Core Code

The Solana plugin uses a custom indexing algorithm, and enables the `CUSTOM_INDEXING` compilation feature when the `SOLANA` feature is enabled. This feature relationship is defined at the bottom of the `etl-core` Cargo.toml file, [found here](https://github.com/BCWResearch/etl-core/blob/main/Cargo.toml).  

When requesting blocks from the node, we avoid requesting blocks at "skipped slots" (see official Solana documentation, [found here](https://docs.solana.com/terminology#skipped-slot)).

To do this, we make use of the `getBlocks` (notice the 's') JSON RPC method. Official Solana documentation for this [found here](https://docs.solana.com/api/http#getblocks). 

With our custom indexing approach, we call `getBlockHeight` to get the current block height, then call `getBlocks` up to the current block height from the last slot we indexed, and then use the non-skipped slots returned by `getBlocks` to make our `getBlock` (notice there is no 's') requests.

NOTE: the `getBlocks` method is only able to request the non-skipped slots of up to 500,000 slots at a time. So, we always check if the number of blocks up to the block height is greater than 500,000. If so, we limit our request to a range 500,000 slots.

# Concurrency

The main thread continually makes RPC requests to `getBlockHeight` and `getBlocks`, to maintain an ongoing list of slots that were not skipped. This thread continually pushes these slots for indexing to a concurrent queue.

At startup, we spawn several (a user-determined amount) of long-lived threads. These threads continually pull slot values from the concurrent queue. With each slot, the threads make `getBlock` requests to the node, deserialize the responses, transform the data into a structure that matches the protocol buffers interface, and finally stream the data to the RabbitMQ Stream Queue. The Stream Queue itself is also a concurrent queue, so the threads do not need to coordinate when they send data.

If writing the data to a CSV file, then a single thread is used for file writing. Another concurrent queue is shared among the CSV writer thread and the indexing threads. The indexing threads send their finished tasks to this queue, and the CSV writer thread pulls data from this queue for writing.
