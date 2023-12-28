package main

import (
	"context"
	"math/big"

	"cloud.google.com/go/bigquery"
	"github.com/apache/beam/sdks/v2/go/pkg/beam/log"
)

func int64PointerToNullInt64(i *int64) bigquery.NullInt64 {
	if i == nil {
		return bigquery.NullInt64{Int64: 0, Valid: false}
	} else {
		return bigquery.NullInt64{Int64: *i, Valid: true}
	}

}

func int64ToNullInt64(i int64) bigquery.NullInt64 {
	return bigquery.NullInt64{Int64: i, Valid: true}
}

func int64ToBigRat(i int64) *big.Rat {
	big_rat := big.NewRat(1, 1)
	return big_rat.SetInt64(i)
}

func uint64ToBigRat(i uint64) *big.Rat {
	big_rat := big.NewRat(1, 1).SetUint64(i)
	return big_rat
}

func uint64PointerToBigRat(i *uint64) *big.Rat {
	if i == nil {
		return nil
	} else {
		return big.NewRat(1, 1).SetUint64(*i)
	}
}

func stringPointerToNullString(s *string) bigquery.NullString {
	if s == nil {
		return bigquery.NullString{StringVal: "", Valid: false}
	} else {
		return bigquery.NullString{StringVal: *s, Valid: *s != ""}
	}
}

func stringPointerToBigRat(s *string) *big.Rat {
	if s == nil {
		return nil
	} else {
		big_rat := new(big.Rat)
		res, _ := big_rat.SetString(*s)
		return res
	}

}

func stringToNullString(s string) bigquery.NullString {
	return bigquery.NullString{StringVal: s, Valid: s != ""}
}

func boolPointerToNullBool(b *bool) bigquery.NullBool {
	if b == nil {
		return bigquery.NullBool{Bool: false, Valid: false}
	} else {
		return bigquery.NullBool{Bool: *b, Valid: true}
	}
}

func boolToNullBool(b bool) bigquery.NullBool {
	return bigquery.NullBool{Bool: b, Valid: true}
}

func stringToBigRat(s string) *big.Rat {
	if s == "" {
		return nil
	} else {
		big_rat := big.NewRat(1, 1)
		if _, ok := big_rat.SetString(s); !ok {
			log.Fatal(context.Background(), "Failed to parse the string")
		}
		return big_rat
	}
}
