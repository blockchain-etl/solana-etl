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
			panic("terminating...")
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
func BatchInsertRecords[RecordType ProtoRecord](ctx context.Context, managedStream *managedwriter.ManagedStream, result chan error, recordsChan <-chan []RecordType) {
	timeout := time.After(30 * time.Second) // TODO: make this timer an environment variable
	curOffset := 0
	//	var results []*managedwriter.AppendResult
	i := 0
loop:
	for {
		select {

		case <-timeout:
			fmt.Println("30 seconds have passed, finalizing rows...")
			break loop
		case msg := <-recordsChan:

			numRecords := len(msg)
			if numRecords == 0 {
				continue
			} else {
				serializedRecords := make([][]byte, numRecords)
				for j, rec := range msg {

					serializedRecord, err := proto.Marshal(rec)
					if err != nil {
						fmt.Println("failed to serialize the block record:", err)
						panic("terminating...")
					}

					serializedRecords[j] = serializedRecord
				}
				var err error
				var result *managedwriter.AppendResult
				// result := nil
				if numRecords > 500 {
/*					midpoint := numRecords / 2
					firstHalf := serializedRecords[midpoint:]
					halfOffset := len(firstHalf)
					secondHalf := serializedRecords[:midpoint]
					fmt.Println("0attempting first appendrows due to large batch")
					result, err := managedStream.AppendRows(ctx, firstHalf, managedwriter.WithOffset(int64(curOffset)))
					if err != nil {
						fmt.Println("0reattempt returned error, before GetResult:", err)
					}
					fmt.Println("0getting result for first reattempt...")
					_, err = result.GetResult(ctx)
					fmt.Println("0got the first result")
					if err != nil {
						fmt.Println("0first reattempted appendrows returned error:", err)
					}
					fmt.Println("0second reattempt after split...")
					result, err = managedStream.AppendRows(ctx, secondHalf, managedwriter.WithOffset(int64(curOffset)+int64(halfOffset)))
*/
fmt.Println("inserting a large batch in segments. numRecords=", numRecords)
/*
partSize := numRecords / 10

for i := 0; i < 10; i++ {
    start := i * partSize
    end := start + partSize

    // Adjust the end for the last slice to include any remaining elements
    if i == 9 {
        end = numRecords
    }

    part := serializedRecords[start:end]
    fmt.Printf("%dth attempt due to large batch\n", i)
    
    offset := int64(start + curOffset)
    result, err = managedStream.AppendRows(ctx, part, managedwriter.WithOffset(offset))
    if err != nil {
        fmt.Printf("%dth reattempt returned error, before GetResult: %v\n", i, err)

    }

    fmt.Printf("%dth getting result for reattempt...\n", i)
    _, err = result.GetResult(ctx)
    if err != nil {
        fmt.Printf("%dth reattempted appendrows returned error: %v\n", i, err)
    }
}
*/

    const maxSliceSize = 500
    // Calculate the number of slices needed
    numSlices := (numRecords + maxSliceSize - 1) / maxSliceSize

    for i := 0; i < numSlices; i++ {
        start := i * maxSliceSize
        end := start + maxSliceSize

        // Adjust the end for the last slice to include any remaining elements
        if end > numRecords {
            end = numRecords
        }

        part := serializedRecords[start:end]
        fmt.Printf("%dth attempt due to large batch\n", i)

        //offset := int64(start + curOffset)
        result, err := managedStream.AppendRows(ctx, part, managedwriter.WithOffset(int64(curOffset)))
        if err != nil {
            fmt.Printf("%dth reattempt returned error, before GetResult: %v\n", i, err)
            // Optionally, you could add logic here to retry or handle the error
            continue
        }

        fmt.Printf("%dth getting result for reattempt...\n", i)
        _, err = result.GetResult(ctx)
        if err != nil {
            fmt.Printf("%dth reattempted appendrows returned error: %v\n", i, err)
            // Again, error handling logic could be added here
        }
        curOffset += len(part)
    }

				} else {
fmt.Println("inserting a single batch. numRows =", numRecords)
					result, err = managedStream.AppendRows(ctx, serializedRecords, managedwriter.WithOffset(int64(curOffset)))

                                        for err != nil {
                                                fmt.Println("AppendRows failed:", err)
                                                result, err = managedStream.AppendRows(ctx, serializedRecords, managedwriter.WithOffset(int64(curOffset)))
                                        }
                                        _, err := result.GetResult(ctx)
                                        if err == nil {
                                                curOffset += numRecords
                                                //break
                                        } else {
					fmt.Println("the error is:", err)
//panic("unrecoverable. panicking...")
					}				
}

/*				for {
					for err != nil {
						fmt.Println("AppendRows failed:", err)
						result, err = managedStream.AppendRows(ctx, serializedRecords, managedwriter.WithOffset(int64(curOffset)))
					}
					_, err := result.GetResult(ctx)
					if err == nil {
						curOffset += numRecords
						break
					}
					fmt.Println("appendrows returned error:", err)
panic()					
*/

/*
midpoint := numRecords / 2
					firstHalf := serializedRecords[midpoint:]
					halfOffset := len(firstHalf)
					secondHalf := serializedRecords[:midpoint]
					fmt.Println("first reattempt after split...")
					result, err = managedStream.AppendRows(ctx, firstHalf, managedwriter.WithOffset(int64(curOffset)))
					if err != nil {
						fmt.Println("reattempt returned error, before GetResult:", err)
					}
					fmt.Println("getting result for first reattempt...")
					_, err = result.GetResult(ctx)
					fmt.Println("got the first result")
					if err != nil {
						fmt.Println("first reattempted appendrows returned error:", err)
					}
					fmt.Println("second reattempt after split...")
                                        curOffset += halfOffset
					result, err = managedStream.AppendRows(ctx, secondHalf, managedwriter.WithOffset(int64(curOffset)))
					fmt.Println("getting result for second reattempt")
					_, err = result.GetResult(ctx)
					fmt.Println("got the second result")
					if err != nil {
						fmt.Println("second reattempted appendrows returned error:", err)
					}
                                       curOffset += len(secondHalf)

					break

				}
				//results = append(results, result)
				i += 1

				if i >= constants.BatchSize {
					break loop
				}
*/
                                i += 1

                                if i >= constants.BatchSize {
                                        break loop
                                }
			}
		}
	}

	// blocks until we receive a response from the API.
	//fmt.Println("checking appendrows results...")
	//checkResults(ctx, results)

	rowCount, err := managedStream.Finalize(ctx)
	for err != nil {
		fmt.Println("error during Finalize:", err)
		rowCount, err = managedStream.Finalize(ctx)
		//		panic("terminating...")
	}
	fmt.Println("Stream finalized with row count:", rowCount)

	result <- err
	managedStream.Close()
}

func StartPipeline[RecordType ProtoRecord](ctx context.Context, writeClient *managedwriter.Client, tableID string, descriptorProto *descriptorpb.DescriptorProto, recordsChan <-chan []RecordType) {
	fmt.Println("starting the pipeline for", tableID)
	ProjectID := os.Getenv("BQ_PROJECT_ID")
	DatasetID := os.Getenv("BQ_DATASET_ID")
	table := fmt.Sprintf("projects/%s/datasets/%s/tables/%s", ProjectID, DatasetID, tableID)
	for {
		streams := make([]string, constants.InserterCount)
		results := make([]chan error, constants.InserterCount)
		for i := 0; i < constants.InserterCount; i++ {
			fmt.Println("creating a writestream for the worker")
			pendingStream, err := writeClient.CreateWriteStream(ctx, &storagepb.CreateWriteStreamRequest{
				Parent: table,
				WriteStream: &storagepb.WriteStream{
					Type: storagepb.WriteStream_PENDING,
				},
			})
			if err != nil {
				fmt.Println("Failed to create a pending stream:", err)
			} else {
				fmt.Println("successfully created the pending stream")
			}
			managedStream, err := writeClient.NewManagedStream(ctx,
				managedwriter.WithStreamName(pendingStream.GetName()),
				managedwriter.WithSchemaDescriptor(descriptorProto),
				managedwriter.EnableWriteRetries(true),
			)
			if err != nil {
				fmt.Println("NewManagedStream:", err)
			} else {
				fmt.Println("managedstream was created successfully")
			}
			result := make(chan error)
			results[i] = result
			fmt.Println("starting a pipeline worker for table", tableID)
			go BatchInsertRecords(ctx, managedStream, result, recordsChan)

			streams[i] = managedStream.StreamName()
		}

		req := &storagepb.BatchCommitWriteStreamsRequest{
			Parent:       table,
			WriteStreams: streams,
		}
		for i := 0; i < constants.InserterCount; i++ {
			err := <-results[i]
			if err != nil {
				panic("append failed. terminating...")
			}
		}
		resp, err := writeClient.BatchCommitWriteStreams(ctx, req)
		if err != nil {
			fmt.Println("failed to commit the stream:", err)
			panic("terminating...")
		} else {
			fmt.Println("Successfully committed at:", resp.GetCommitTime().AsTime().Format(time.RFC3339Nano))
		}
	}
}
