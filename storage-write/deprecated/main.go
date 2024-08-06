//go:generate bash ./run_protoc.sh

package main

import (
	"context"
	"fmt"
	"os"
	"subscriber_rabbitmq/constants"
	"subscriber_rabbitmq/insertion"
	"subscriber_rabbitmq/pbcodegen"
	"subscriber_rabbitmq/transformation"

	"github.com/apache/beam/sdks/v2/go/pkg/beam/log"
	"github.com/joho/godotenv"

	amqp "github.com/rabbitmq/amqp091-go"
	//"github.com/rabbitmq/rabbitmq-stream-go-client/pkg/amqp"

	"cloud.google.com/go/bigquery"
	"cloud.google.com/go/bigquery/storage/managedwriter"
	"cloud.google.com/go/bigquery/storage/managedwriter/adapt"

	//"cloud.google.com/go/pubsub"
	_ "github.com/apache/beam/sdks/v2/go/pkg/beam/io/filesystem/local"
	"github.com/klauspost/compress/zstd"
	"google.golang.org/protobuf/proto"
)

func main() {
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
	QueueName := os.Getenv("QUEUE_NAME")

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

	//err = ch.Qos(1, 0, false) // prefetch count, prefetch size, global

	// assumes the queue is declared by the producer

	msgs, err := ch.Consume(
		QueueName,  // queue
		os.Args[1], // consumer
		true,       // auto-ack
		false,      // exclusive
		false,      // no-local
		false,      // no-wait
		nil,        // args
	)

	if err != nil {
		fmt.Printf("Failed to create a rabbitmq consumer: %v", err)
	}

	forever := make(chan bool)

	// create internal queues (table-specific) for insertion into BQ
	blockRecordsChan := make(chan []*pbcodegen.BlockRecord, constants.BatchSize)
	blockRewardRecordsChan := make(chan []*pbcodegen.BlockRewardRecord, constants.BatchSize)
	transactionRecordsChan := make(chan []*pbcodegen.TransactionRecord, constants.BatchSize)
	instructionRecordsChan := make(chan []*pbcodegen.InstructionRecord, constants.BatchSize)
	tokenTransferRecordsChan := make(chan []*pbcodegen.TokenTransferRecord, constants.BatchSize)
	accountRecordsChan := make(chan []*pbcodegen.AccountRecord, constants.BatchSize)
	tokenRecordsChan := make(chan []*pbcodegen.TokenRecord, constants.BatchSize)

	// some setup for the write client
	writeClient, err := managedwriter.NewClient(ctx, ProjectID)
	if err != nil {
		fmt.Println("Failed to create a BQ write client:", err)
	}
	defer writeClient.Close()

	// each pipeline (table-specific) will pull table records from the internal queue, and insert batches in to BQ
	b := &pbcodegen.BlockRecord{}
	blockDescriptorProto, err := adapt.NormalizeDescriptor(b.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	br := &pbcodegen.BlockRewardRecord{}
	blockRewardsDescriptorProto, err := adapt.NormalizeDescriptor(br.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	tra := &pbcodegen.TransactionRecord{}
	transactionDescriptorProto, err := adapt.NormalizeDescriptor(tra.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	ins := &pbcodegen.InstructionRecord{}
	instructionDescriptorProto, err := adapt.NormalizeDescriptor(ins.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	tt := &pbcodegen.TokenTransferRecord{}
	tokenTransferDescriptorProto, err := adapt.NormalizeDescriptor(tt.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	tok := &pbcodegen.TokenRecord{}
	tokenDescriptorProto, err := adapt.NormalizeDescriptor(tok.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	acc := &pbcodegen.AccountRecord{}
	accountDescriptorProto, err := adapt.NormalizeDescriptor(acc.ProtoReflect().Descriptor())
	if err != nil {
		fmt.Println("NormalizeDescriptor:", err)
	}

	go insertion.StartPipeline(ctx, writeClient, constants.BlockTableID, blockDescriptorProto, blockRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.BlockRewardsTableID, blockRewardsDescriptorProto, blockRewardRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.TransactionTableID, transactionDescriptorProto, transactionRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.InstructionTableID, instructionDescriptorProto, instructionRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.TokenTransferTableID, tokenTransferDescriptorProto, tokenTransferRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.TokenTableID, tokenDescriptorProto, tokenRecordsChan)

	go insertion.StartPipeline(ctx, writeClient, constants.AccountTableID, accountDescriptorProto, accountRecordsChan)

	// deserialize the raw data from rabbitmq and spawn transformer threads.
	// the transformed records are then sent by the worker threads to an internal queue for insertion
	go func() {
		for d := range msgs {
			d := d
			go func() {

				deserializedMsg := deserializeMsg(d.Body)
				if deserializedMsg == nil {
					panic("failed to deserialize message")
				}

				if deserializedMsg.Block != nil {
					blockRecord := transformation.TransformToBlockRecord(deserializedMsg)
					if blockRecord != nil {
						blockRecordsChan <- blockRecord
					}
				}

				if deserializedMsg.BlockRewards != nil {
					blockRewardRecords := transformation.TransformToBlockRewardRecords(deserializedMsg)
					if blockRewardRecords != nil {
						blockRewardRecordsChan <- blockRewardRecords
					}
				}

				if deserializedMsg.Transactions != nil {
					transactionRecords, instructionRecords, tokenTransferRecords := transformation.TransformToTransactionRecords(deserializedMsg)
					if transactionRecords != nil {
						transactionRecordsChan <- transactionRecords
					}

					if instructionRecords != nil {
						instructionRecordsChan <- instructionRecords
					}

					if tokenTransferRecords != nil {
						tokenTransferRecordsChan <- tokenTransferRecords
					}

				}

				if deserializedMsg.Accounts != nil {

					accountRecords, tokenRecords := transformation.TransformToAccountRecords(deserializedMsg)

					if accountRecords != nil {
						accountRecordsChan <- accountRecords
					}

					if tokenRecords != nil {
						tokenRecordsChan <- tokenRecords
					}

				}

				//fmt.Println("Finished processing the message, ending the thread.")
			}()

		}
	}()

	fmt.Println(" [*] Waiting for messages. To exit press CTRL+C")
	<-forever
}

func decompressMsg(msg []uint8) []uint8 {
	var decompressedData []uint8
	// NOTE: it is not safe to reuse the same decoder across multiple goroutines,
	// so we create a new one in each function call
	d, err := zstd.NewReader(nil)
	if err != nil {
		log.Fatal(context.Background(), "failed to create a zstd decompressor", err)
	}
	defer d.Close()

	decompressedData, err = d.DecodeAll(msg, nil)
	if err != nil {
		log.Fatal(context.Background(), "failed to decompress", err)
	}
	return decompressedData
}

func deserializeMsg(msg []uint8) *pbcodegen.EtlBlock {
	var rawBlock pbcodegen.EtlBlock

	if err := proto.Unmarshal(msg, &rawBlock); err != nil {
		fmt.Println("proto.Unmarshal err:", err)
	}

	return &rawBlock
}
