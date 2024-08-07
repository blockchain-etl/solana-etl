locals {
  roles = [
    "roles/storage.objectViewer",
    "roles/logging.logWriter",
    "roles/monitoring.metricWriter"
    # Add more roles as needed
  ]

  app_roles = [
    "roles/bigquery.dataEditor"
  ]

}
resource "google_service_account" "solana_etl" {
  account_id   = "compute-service-account"
  display_name = "Compute Service Account"
  project      = var.project_id
}

resource "google_project_iam_member" "service_account_roles" {
  for_each = toset(local.roles)

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.solana_etl.email}"
}

resource "google_service_account" "app_sa" {
  account_id   = "solana-app-sa"
  display_name = "Solana Apps Account"
}

resource "google_project_iam_member" "apps_sa_roles" {
  for_each = toset(local.app_roles)

  project = var.project_id
  role    = each.value
  member  = "serviceAccount:${google_service_account.app_sa.email}"
}

