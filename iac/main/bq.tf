locals {
  schema_names = [
    "Accounts",
    "Block Rewards",
    "Blocks",
    "Instructions",
    "Token Transfers",
    "Tokens",
    "Transactions"
  ]

  schemas_and_datasets = [
    for name in local.schema_names : {
      dataset_name = "crypto_solana_mainnet_us"
      table_name   = name
      schema_file  = name
    }
  ]
}

resource "google_bigquery_dataset" "solana_dataset" {
  dataset_id                  = "crypto_solana_mainnet_us"
  location                    = "US"
  default_table_expiration_ms = 21600000
}


resource "google_bigquery_table" "solana_tables" {
  for_each            = { for item in local.schemas_and_datasets : "${item.dataset_name}.${item.table_name}" => item }
  dataset_id          = each.value.dataset_name
  table_id            = each.value.table_name
  deletion_protection = false

  schema = file("${path.module}/schemas/${each.value.schema_file}.json")

  depends_on = [
    google_bigquery_dataset.solana_dataset
  ]
}
