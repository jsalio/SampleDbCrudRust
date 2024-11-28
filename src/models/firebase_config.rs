
// FirebaseConfig model
#[derive(serde::Deserialize, Debug)]
pub struct FirebaseConfig {
       pub api_key: String,
       pub  auth_domain: String,
       pub database_url: String,
       pub project_id: String,
       pub storage_bucket: String,
       pub messaging_sender_id: String,
       pub app_id: String,
}