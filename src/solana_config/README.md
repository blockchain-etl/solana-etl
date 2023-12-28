# etl-solana-config
This repository is a plugin for the `etl-core` indexer framework. Here, all of the necessary Solana details are included, such as RPC calls, JSON response deserialization, and the protocol buffers interface.
The `etl-core` repository imports this repository as a git submodule into `src/`, named `solana_config`, and uses conditional compilation through Rust's feature system to import this code as needed.
* Note that this repository *must* be renamed this way, because Rust does not support importing a module with the `-` character in the directory name. Submodule renaming is [done here](https://github.com/BCWResearch/etl-core/blob/main/.gitmodules). 

# System Setup
This is not specific to Solana. See [this section](https://github.com/BCWResearch/etl-core/tree/main/#setup) of the indexer core documentation.

# Build
## For Deployment
Clone the `etl-core` repository with its submodules, and enter the indexer directory:

```
git clone --recurse-submodules --remote-submodules https://github.com/BCWResearch/etl-core.git
cd etl-core/indexer/
```

Then, include this repository for compilation by enabling the `SOLANA` feature, and compile with optimizations:

`cargo build --release --features SOLANA`
* The resulting binary can be found in `etl-core/indexer/target/release/`
* Logging be configured using the `RUST_LOG` environment variable (values are `error`, `warn`, `info`, `debug`, `trace`).

## For Development
Clone the `etl-core` repository with its submodules, and enter the indexer directory:

```
git clone --recurse-submodules --remote-submodules https://github.com/BCWResearch/etl-core.git
cd etl-core/indexer/
```

Then, build and run the development profile (default) and enable the SOLANA feature:

`cargo run --features SOLANA`
* Logging can be configured using the `RUST_LOG` environment variable (values are `error`, `warn`, `info`, `debug`, `trace`).
* For faster builds, you may choose to install and use the `mold` linker: https://github.com/rui314/mold
