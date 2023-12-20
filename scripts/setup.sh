sudo apt install git cargo g++ protobuf-compiler
git clone https://github.com/blockchain-etl/solana-etl.git
cd solana-etl
cargo build –-release --features RABBITMQ_CLASSIC
