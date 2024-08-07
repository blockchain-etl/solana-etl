terraform {
  required_version = "~> 1.5"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.40.0"
    }
  }
  backend "gcs" {
    bucket = <your bucket name>
    prefix = "solana-bq/bq"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}
provider "google-beta" {
  project = var.project_id
  region  = var.region
}

data "google_client_config" "this" {}
data "google_project" "this" {}
