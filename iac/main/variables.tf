locals {
  scopes = [
    "https://www.googleapis.com/auth/devstorage.read_only",
    "https://www.googleapis.com/auth/logging.write",
    "https://www.googleapis.com/auth/monitoring.write",
    "https://www.googleapis.com/auth/service.management.readonly",
    "https://www.googleapis.com/auth/servicecontrol",
    "https://www.googleapis.com/auth/trace.append"
  ]
}

variable "project_id" {
  default = "PROJECT-408009"
}
variable "region" {
  description = "The zone for the instance"
  default     = "us-east4"
}
variable "cird_range" {
  default = "10.0.0.0/24"
}
