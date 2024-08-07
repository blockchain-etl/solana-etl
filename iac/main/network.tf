resource "google_compute_network" "solana_etl" {
  auto_create_subnetworks = false
  mtu                     = 1460
  name                    = "${var.project_id}-solala-bq-vpc"
  project                 = var.project_id
  routing_mode            = "REGIONAL"
}
resource "google_compute_subnetwork" "solana_etl" {
  ip_cidr_range              = var.cird_range
  name                       = "${var.project_id}-solala-bq-subnet"
  network                    = google_compute_network.solana_etl.id
  private_ip_google_access   = true
  private_ipv6_google_access = "DISABLE_GOOGLE_ACCESS"
  project                    = var.project_id
  purpose                    = "PRIVATE"
  region                     = var.region
  stack_type                 = "IPV4_ONLY"
}
resource "google_compute_address" "solana_rpc_public" {
  address_type = "EXTERNAL"
  name         = "solana-rpc-public"
  network_tier = "STANDARD"
  project      = var.project_id
  region       = var.region
}
resource "google_compute_address" "solana_rpc_internal" {
  address      = "10.0.0.12"
  address_type = "INTERNAL"
  name         = "solana-rpc-internal"
  project      = var.project_id
  region       = var.region
  subnetwork   = google_compute_subnetwork.solana_etl.name
}

resource "google_compute_address" "rabbit_mq_internal" {
  address      = "10.0.0.3"
  address_type = "INTERNAL"
  name         = "rabbit-mq-internal"
  project      = var.project_id
  region       = var.region
  subnetwork   = google_compute_subnetwork.solana_etl.name
}

resource "google_compute_address" "inserter_internal" {
  address      = "10.0.0.4"
  address_type = "INTERNAL"
  name         = "inserter-internal"
  project      = var.project_id
  region       = var.region
  subnetwork   = google_compute_subnetwork.solana_etl.name
}

resource "google_compute_address" "indexer_internal" {
  address      = "10.0.0.5"
  address_type = "INTERNAL"
  name         = "indexer-internal"
  project      = var.project_id
  region       = var.region
  subnetwork   = google_compute_subnetwork.solana_etl.name
}
