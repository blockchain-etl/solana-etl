terraform {
  required_version = "~> 1.5"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.40.0"
    }
  }
  backend "gcs" {
    bucket = <your bucket name> #Bucket name for 
    prefix = "solana-bq/bq"
  }
}

variable "project_id" { ###Name of a freshly created project
  default = <your project id>
}
variable "region" {
  description = "The zone for the instance"
  default     = "us-central1"
}
variable "cird_range" {
  default = "10.0.0.0/24"
}

variable "app_vm_size" {
  description = "The VM size for the applications"
  default     = "n2d-standard-16"
}


locals {
  scopes = [
    "https://www.googleapis.com/auth/devstorage.read_only",
    "https://www.googleapis.com/auth/logging.write",
    "https://www.googleapis.com/auth/monitoring.write",
    "https://www.googleapis.com/auth/service.management.readonly",
    "https://www.googleapis.com/auth/servicecontrol",
    "https://www.googleapis.com/auth/trace.append",
    "https://www.googleapis.com/auth/cloud-platform"
  ]
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
