# Types

In the `types/` directory, there is two source files: `request_types.rs` and `response_types.rs`.

## Requests

The purpose of `request_types.rs` is to construct JSON RPC calls to the Solana node. There are `struct`s in this source file that describe the format of the JSON RPC calls, as well as constructors to make usage of them easier for us - specifically, to hide unused parameters and formatting details from the caller. 

The official Solana documentation on the JSON RPC interface was used as a reference. It can be [found here](https://docs.solana.com/api/http).

## Responses

The purpose of `response_types.rs` is to deserialize the JSON RPC responses, and then to reformat it into a protobuf-compliant data structure for our own serialization. Documentation on our usage of protocol buffers can be found in `protobuf.md`.

We use the official Solana documentation on the JSON RPC methods as a reference when writing our response deserialization code ([found here](https://docs.solana.com/api/http)), as well as our own testing.

For response deserialization, we use the popular Rust library, `serde`. This library allows us to simply write a Rust `struct` that specifies the format of the response, and then use the `Deserialize` macro to generate deserialization code for us.
