resource "google_compute_firewall" "allow_ssh_iap" {
  allow {
    ports    = ["22"]
    protocol = "tcp"
  }

  direction     = "INGRESS"
  name          = "allow-ssh-iap"
  network       = google_compute_network.solana_etl.self_link
  priority      = 1000
  source_ranges = ["35.235.240.0/20"]
}

resource "google_compute_firewall" "solana-rpc" {
  name = "solana-rpc"
  allow {
    ports    = ["8000-10000", "80"]
    protocol = "udp"
  }

  allow {
    ports    = ["8000-10000", "80"]
    protocol = "tcp"
  }

  direction     = "INGRESS"
  network       = google_compute_network.solana_etl.name
  priority      = 1000
  source_ranges = ["0.0.0.0/0"]
  target_tags   = ["solana"]
}

resource "google_compute_firewall" "allow_rabbitmq" {
  name          = "allow-rabbitmq"
  network       = google_compute_network.solana_etl.self_link
  direction     = "INGRESS"
  priority      = 1000
  source_ranges = ["0.0.0.0/0"]

  allow {
    protocol = "tcp"
    ports    = ["5672"]
  }

  target_tags = ["solana"]
}


