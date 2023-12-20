//go:generate bash ./run_protoc.sh

package main

import (
	"context"
	"fmt"
	"os"
	"subscriber_rabbitmq/constants"
	"subscriber_rabbitmq/insertion"
	"subscriber_rabbitmq/pbcodegen"

	"github.com/joho/godotenv"

	amqp "github.com/rabbitmq/amqp091-go"

	"cloud.google.com/go/bigquery"
	"cloud.google.com/go/bigquery/storage/managedwriter"
	"cloud.google.com/go/bigquery/storage/managedwriter/adapt"

	_ "github.com/apache/beam/sdks/v2/go/pkg/beam/io/filesystem/local"
	"google.golang.org/protobuf/proto"
)

func main() {
	// one CLI argument, the type of records to insert:
	recordType := os.Args[1]

	err := godotenv.Load()
	if err != nil {
		fmt.Printf("Error loading .env file")
	}

	ctx, _ := context.WithCancel(context.Background())

	ProjectID := os.Getenv("BQ_PROJECT_ID")
	bq_client, err := bigquery.NewClient(ctx, ProjectID)
	if err != nil {
		fmt.Println("bigquery.NewClient err: %w", err)
	}
	defer bq_client.Close()
	fmt.Println("connected to bigquery")

	fmt.Println("creating the rabbitmq environment")

	RabbitmqUser := os.Getenv("RABBITMQ_USER")
	RabbitmqPassword := os.Getenv("RABBITMQ_PASS")
	RabbitmqHost := os.Getenv("RABBITMQ_HOST")
	RabbitmqPort := os.Getenv("RABBITMQ_PORT")

	conn, err := amqp.Dial(fmt.Sprintf("amqp://%s:%s@%s:%s/", RabbitmqUser, RabbitmqPassword, RabbitmqHost, RabbitmqPort))

	if err != nil {
		fmt.Printf("Failed to connect to rabbitmq server: %v", err)
	}
	defer conn.Close()
	ch, err := conn.Channel()
	if err != nil {
		fmt.Printf("Failed to open a channel to rabbitmq server: %v", err)
	}
	defer ch.Close()

	// assumes the queue has already been declared
	msgs, err := ch.Consume(
		recordType,         // queue
		constants.Consumer, // consumer
		true,               // auto-ack
		false,              // exclusive
		false,              // no-local
		false,              // no-wait
		nil,                // args
	)

	if err != nil {
		fmt.Printf("Failed to create a rabbitmq consumer: %v", err)
	}

	// create internal queues (table-specific) for insertion into BQ
	var blockRecordsChan chan [1]*pbcodegen.BlockRecord
	var blockRewardRecordsChan chan [1]*pbcodegen.BlockRewardRecord
	var transactionRecordsChan chan [1]*pbcodegen.TransactionRecord
	var instructionRecordsChan chan [1]*pbcodegen.InstructionRecord
	var tokenRecordsChan chan [1]*pbcodegen.TokenRecord
	var tokenTransferRecordsChan chan [1]*pbcodegen.TokenTransferRecord
	var accountRecordsChan chan [1]*pbcodegen.AccountRecord

	// some setup for the write client
	writeClient, err := managedwriter.NewClient(ctx, ProjectID)
	if err != nil {
		fmt.Println("Failed to create a BQ write client:", err)
	}
	defer writeClient.Close()

	// each pipeline (table-specific) will pull table records from the internal queue, and insert batches into BQ
	switch recordType {
	case "blocks":
		recordInterface := &pbcodegen.BlockRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		blockRecordsChan = make(chan [1]*pbcodegen.BlockRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.BlockTableID, descriptorProto, blockRecordsChan)
	case "block_rewards":
		recordInterface := &pbcodegen.BlockRewardRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		blockRewardRecordsChan = make(chan [1]*pbcodegen.BlockRewardRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.BlockRewardsTableID, descriptorProto, blockRewardRecordsChan)
	case "transactions":
		recordInterface := &pbcodegen.TransactionRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		transactionRecordsChan = make(chan [1]*pbcodegen.TransactionRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.TransactionTableID, descriptorProto, transactionRecordsChan)
	case "instructions":
		recordInterface := &pbcodegen.InstructionRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		instructionRecordsChan = make(chan [1]*pbcodegen.InstructionRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.InstructionTableID, descriptorProto, instructionRecordsChan)
	case "token_transfers":
		recordInterface := &pbcodegen.TokenTransferRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		tokenTransferRecordsChan = make(chan [1]*pbcodegen.TokenTransferRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.TokenTransferTableID, descriptorProto, tokenTransferRecordsChan)
	case "tokens":
		recordInterface := &pbcodegen.TokenRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		tokenRecordsChan = make(chan [1]*pbcodegen.TokenRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.TokenTableID, descriptorProto, tokenRecordsChan)
	case "accounts":
		recordInterface := &pbcodegen.AccountRecord{}
		descriptorProto, err := adapt.NormalizeDescriptor(recordInterface.ProtoReflect().Descriptor())
		if err != nil {
			fmt.Println("NormalizeDescriptor:", err)
		}
		accountRecordsChan = make(chan [1]*pbcodegen.AccountRecord, constants.BatchSize)
		go insertion.StartPipeline(ctx, writeClient, constants.AccountTableID, descriptorProto, accountRecordsChan)
	default:
		fmt.Println("Unknown record type:", os.Args[1])
		panic("Please provide one of the following: blocks, block_rewards, transactions, instructions, token_transfers, tokens, accounts")
	}
	// deserialize the raw data from rabbitmq and spawn transformer threads.
	// the transformed records are then sent by the worker threads to an internal queue for insertion
	forever := make(chan bool)
	for d := range msgs {
		d := d
		go func() {
			fmt.Println("Received a message, spawning a thread...")
			switch recordType {
			case "blocks":
				fmt.Println("Starting the block record inserter...")
				var recordData pbcodegen.BlockRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.BlockRecord{&recordData}
				blockRecordsChan <- record

			case "block_rewards":
				fmt.Println("Starting the block reward record inserter...")
				var recordData pbcodegen.BlockRewardRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.BlockRewardRecord{&recordData}
				blockRewardRecordsChan <- record

			case "transactions":
				fmt.Println("Starting the transaction record inserter...")
				var recordData pbcodegen.TransactionRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.TransactionRecord{&recordData}
				transactionRecordsChan <- record

			case "instructions":
				fmt.Println("Starting the instruction record inserter...")
				var recordData pbcodegen.InstructionRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.InstructionRecord{&recordData}
				instructionRecordsChan <- record

			case "token_transfers":
				fmt.Println("Starting the token transfer record inserter...")
				var recordData pbcodegen.TokenTransferRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.TokenTransferRecord{&recordData}
				tokenTransferRecordsChan <- record

			case "tokens":
				fmt.Println("Starting the token record inserter...")
				var recordData pbcodegen.TokenRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.TokenRecord{&recordData}
				tokenRecordsChan <- record
			case "accounts":
				fmt.Println("Starting the account record inserter...")
				var recordData pbcodegen.AccountRecord
				if err := proto.Unmarshal(d.Body, &recordData); err != nil {
					fmt.Println("proto.Unmarshal err:", err)
					panic("")
				}

				record := [1]*pbcodegen.AccountRecord{&recordData}
				accountRecordsChan <- record
			default:
				fmt.Println("Unknown record type:", os.Args[1])
				panic("Please provide one of the following: blocks, block_rewards, transactions, instructions, token_transfers, tokens, accounts")
			}

			fmt.Println("Finished processing the message, ending the thread.")
		}()

	}

	fmt.Println(" [*] Waiting for messages. To exit press CTRL+C")
	<-forever
}
