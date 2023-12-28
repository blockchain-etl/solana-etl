# Protocol Buffers

We use a the official "storage" Protocol Buffers interface from the Solana source code, [found here](https://github.com/solana-labs/solana/tree/master/storage-proto), and tweak it to include data from the parsed encoding, as well as to add account and token data.

The Solana code for converting from internal data structures to this "storage" format can be [found here](https://github.com/solana-labs/solana/blob/master/storage-proto/src/convert.rs). Using this as a reference, we attempt to follow their usage as closely as possible when converting from the JSON RPC responses to the protocol buffers interface. See the `types` documentation for more information on this converion.

Solana uses the PROST library to generate Rust code for the interface, so we use it here, too. This code generation happens at compile time, and is specified in the `build_proto.rs` file. This build script is included and executed by the `build_proto.rs` found in `etl-core`, so the file paths are relative to that `etl-core` build script location.  
