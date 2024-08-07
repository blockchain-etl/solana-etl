# RabbitMQ Server Configuration
resource "google_compute_instance" "rabbitmq" {
  name    = "rabbitmq-server"
  project = var.project_id
  zone    = "${var.region}-a"
  # machine_type     = "n2-standard-8"
  machine_type = var.app_vm_size
  # min_cpu_platform = "Intel Ice Lake"

  boot_disk {
    auto_delete = true
    initialize_params {
      image = "https://www.googleapis.com/compute/beta/projects/ubuntu-os-cloud/global/images/ubuntu-minimal-2204-jammy-v20230715"
      size  = 1200
      type  = "pd-balanced"
    }
  }

  network_interface {

    network    = google_compute_network.solana_etl.name
    subnetwork = google_compute_subnetwork.solana_etl.name
    network_ip = google_compute_address.rabbit_mq_internal.address

  }

  scheduling {
    automatic_restart   = true
    on_host_maintenance = "MIGRATE"
    provisioning_model  = "STANDARD"
  }

  service_account {
    email  = google_service_account.solana_etl.email
    scopes = local.scopes
  }

  shielded_instance_config {
    enable_integrity_monitoring = true
    enable_vtpm                 = true
  }

  metadata_startup_script = file("../scripts/rabbit-mq/run-rabbit.sh")

  tags = ["solana"]
}

resource "google_compute_instance" "inserter" {
  name         = "bq-inserter"
  project      = var.project_id
  zone         = "${var.region}-a"
  machine_type = var.app_vm_size

  boot_disk {
    auto_delete = true
    initialize_params {
      image = "https://www.googleapis.com/compute/beta/projects/ubuntu-os-cloud/global/images/ubuntu-minimal-2204-jammy-v20230715"
      size  = 200
      type  = "pd-balanced"
    }
  }

  network_interface {

    network    = google_compute_network.solana_etl.name
    subnetwork = google_compute_subnetwork.solana_etl.name
    network_ip = google_compute_address.inserter_internal.address

  }

  scheduling {
    automatic_restart   = true
    on_host_maintenance = "MIGRATE"
    provisioning_model  = "STANDARD"
  }

  service_account {
    email  = google_service_account.app_sa.email
    scopes = local.scopes
  }

  shielded_instance_config {
    enable_integrity_monitoring = true
    enable_vtpm                 = true
  }

  metadata_startup_script = file("../scripts/inserter/inserter-service.sh")
  tags                    = ["solana"]
}

resource "google_compute_instance" "indexer" {
  name         = "bq-indexer"
  project      = var.project_id
  zone         = "${var.region}-a"
  machine_type = var.app_vm_size

  boot_disk {
    auto_delete = true
    initialize_params {
      image = "https://www.googleapis.com/compute/beta/projects/ubuntu-os-cloud/global/images/ubuntu-minimal-2204-jammy-v20230715"
      size  = 200
      type  = "pd-balanced"
    }
  }

  network_interface {

    network    = google_compute_network.solana_etl.name
    subnetwork = google_compute_subnetwork.solana_etl.name
    network_ip = google_compute_address.indexer_internal.address

  }

  scheduling {
    automatic_restart   = true
    on_host_maintenance = "MIGRATE"
    provisioning_model  = "STANDARD"
  }

  service_account {
    email  = google_service_account.app_sa.email
    scopes = local.scopes
  }

  shielded_instance_config {
    enable_integrity_monitoring = true
    enable_vtpm                 = true
  }

  metadata_startup_script = file("../scripts/indexer/indexer-service.sh")
  tags                    = ["solana"]
}



resource "google_compute_instance" "solana_rpc" {
  name             = "solana-rpc"
  project          = var.project_id
  zone             = "${var.region}-a"
  machine_type     = "n2-highmem-64"
  min_cpu_platform = "Intel Ice Lake"

  boot_disk {
    auto_delete = true
    initialize_params {
      image = "https://www.googleapis.com/compute/beta/projects/debian-cloud/global/images/debian-11-bullseye-v20231010"
      size  = 500
      type  = "pd-ssd"
    }
  }

  network_interface {
    network    = google_compute_network.solana_etl.name
    subnetwork = google_compute_subnetwork.solana_etl.name
    access_config {
      nat_ip       = google_compute_address.solana_rpc_public.address
      network_tier = "STANDARD"
    }
    network_ip = google_compute_address.solana_rpc_internal.address
  }

  scheduling {
    automatic_restart   = true
    on_host_maintenance = "MIGRATE"
    provisioning_model  = "STANDARD"
  }

  service_account {
    email  = google_service_account.solana_etl.email
    scopes = local.scopes
  }

  shielded_instance_config {
    enable_integrity_monitoring = true
    enable_vtpm                 = true
  }

  dynamic "scratch_disk" {
    for_each = range(16)
    content {
      interface = "NVME"
    }
  }
  tags = ["solana"]
}
