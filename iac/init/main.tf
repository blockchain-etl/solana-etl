terraform {
  required_version = "~> 1.5"
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.2"
    }
  }
}


variable "region" {
  default = "us-east"
}

variable "project" {
    default =
}

locals {
  project_id = ""
  region     = ""
  env        = "shared"
  default_labels = {
    env        = local.env
    managed-by = "terraform"
  }
  activate_apis = [
    "storage.googleapis.com",
    "iam.googleapis.com",
    "compute.googleapis.com",
    "iap.googleapis.com",
  ]
}

provider "google" {
  project = local.project_id
  region  = local.region
}

provider "google-beta" {
  project = local.project_id
  region  = local.region
}


data "google_project" "this" {}

data "google_compute_default_service_account" "default" {}

resource "google_project_service" "enabled" {
  for_each = toset(local.activate_apis)
  project  = local.project_id
  service  = each.key

  timeouts {
    create = "30m"
    update = "40m"
  }

  disable_on_destroy         = true
  disable_dependent_services = true
}
