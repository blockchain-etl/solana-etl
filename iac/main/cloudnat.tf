resource "google_compute_router" "solana_bq_router" {
  name    = "solana-bq-router"
  network = google_compute_network.solana_etl.self_link
  region  = var.region
}

resource "google_compute_router_nat" "solana_bq_nat_config" {
  name                               = "solana-bq-nat-config"
  router                             = google_compute_router.solana_bq_router.name
  region                             = var.region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
}
