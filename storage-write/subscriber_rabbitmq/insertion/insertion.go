package insertion

import (
	"context"
	"fmt"
	"os"
	"subscriber_rabbitmq/constants"
	"time"

	"cloud.google.com/go/bigquery/storage/apiv1/storagepb"
	"cloud.google.com/go/bigquery/storage/managedwriter"
	"google.golang.org/protobuf/proto"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/types/descriptorpb"
)

func checkResults(ctx context.Context, results []*managedwriter.AppendResult) {
	for _, v := range results {
		_, err := v.GetResult(ctx)
		if err != nil {
			fmt.Println("append returned error:", err)
			panic("")
		} else {
			fmt.Println("Successfully appended data at with result: ", v)
		}
	}
}

type ProtoRecord interface {
	protoreflect.ProtoMessage
}

// generic function for inserting any kind of record.
// appends after the batch size has been reached, or if 30 seconds have passed
func BatchInsertRecords[RecordType ProtoRecord](ctx context.Context, managedStream *managedwriter.ManagedStream, result chan error, recordsChan <-chan [1]RecordType) {
	BatchSize := os.Getenv("BATCH_SIZE")
	timeout := time.After(30 * time.Second)
	curOffset := 0
	var results []*managedwriter.AppendResult
	i := 0
loop:
	for {
		select {

		case <-timeout:
			fmt.Println("30 seconds have passed, finalizing records...")
			break loop
		case msg := <-recordsChan:
			numRecords := len(msg)
			serializedRecords := make([][]byte, numRecords)
			for j, rec := range msg {

				serializedRecord, err := proto.Marshal(rec)
				if err != nil {
					fmt.Println("failed to serialize the record:", err)
					panic("")
				}

				serializedRecords[j] = serializedRecord
			}

			result, err := managedStream.AppendRows(ctx, serializedRecords, managedwriter.WithOffset(int64(curOffset)))
			if err != nil {
				fmt.Println("AppendRows failed:", err)
				panic("")
			}
			results = append(results, result)
			curOffset += numRecords
			i += 1

			if i >= BatchSize {
				break loop
			}

		}
	}

	// blocks until we receive a response from the API.
	checkResults(ctx, results)

	rowCount, err := managedStream.Finalize(ctx)
	if err != nil {
		fmt.Println("error during Finalize:", err)
		panic("terminating...")
	} else {
		fmt.Println("Stream finalized with row count:", rowCount)
	}

	result <- err
	managedStream.Close()
}

func StartPipeline[RecordType ProtoRecord](ctx context.Context, writeClient *managedwriter.Client, tableID string, descriptorProto *descriptorpb.DescriptorProto, recordsChan <-chan [1]RecordType) {
	ProjectID := os.Getenv("BQ_PROJECT_ID")
	DatasetID := os.Getenv("BQ_DATASET_ID")
	InserterCount := os.Getenv("INSERTER_COUNT")
	table := fmt.Sprintf("projects/%s/datasets/%s/tables/%s", ProjectID, DatasetID, tableID)
	for {
		streams := make([]string, InserterCount)
		results := make([]chan error, InserterCount)
		for i := 0; i < InserterCount; i++ {
			pendingStream, err := writeClient.CreateWriteStream(ctx, &storagepb.CreateWriteStreamRequest{
				Parent: table,
				WriteStream: &storagepb.WriteStream{
					Type: storagepb.WriteStream_PENDING,
				},
			})
			if err != nil {
				fmt.Println("Failed to create a pending stream:", err)
			}
			managedStream, err := writeClient.NewManagedStream(ctx,
				managedwriter.WithStreamName(pendingStream.GetName()),
				managedwriter.WithSchemaDescriptor(descriptorProto),
			)
			if err != nil {
				fmt.Println("NewManagedStream:", err)
			}
			result := make(chan error)
			results[i] = result
			go BatchInsertRecords(ctx, managedStream, result, recordsChan)

			streams[i] = managedStream.StreamName()
		}

		req := &storagepb.BatchCommitWriteStreamsRequest{
			Parent:       table,
			WriteStreams: streams,
		}
		for i := 0; i < InserterCount; i++ {
			err := <-results[i]
			if err != nil {
				panic("append failed")
			}
		}
		resp, err := writeClient.BatchCommitWriteStreams(ctx, req)
		if err != nil {
			fmt.Println("failed to commit the stream:", err)
			panic("")
		} else {
			fmt.Println("Successfully committed at:", resp.GetCommitTime().AsTime().Format(time.RFC3339Nano))
		}
	}
}
