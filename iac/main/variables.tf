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

variable "project_id" {
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
