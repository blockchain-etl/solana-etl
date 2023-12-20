# Solana-ETL Inserter

This pulls messages from a RabbitMQ message queue, and uses BigQuery Storage Write API to write records to a Solana dataset in Google BigQuery. 

# System Setup
```
sudo apt install g++ protobuf-compiler python3 golang-go
go get google.golang.org/grpc/cmd/protoc-gen-go-grpc
```

# Compile
```
cd subscriber_rabbitmq/
```

Compile the protos:
```
go generate
```

Compile the Go code:
```
go build
```

# Run
Ensure that you have set up the `.env` file - see [the documentation](/docs/env.md).

To run inserters for all tables:
```
python run_all.py
```

Logs will be in the root.
