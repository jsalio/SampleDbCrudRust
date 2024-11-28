use crate::models::configuration::Configuration;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardInfo {
    pub client_name: String,
    pub configuration: Vec<Configuration>,
    pub creation_date: String,
    pub creator_email: String,
    pub creator_name: String,
    pub creator_phone: String,
    pub current_document_in_process: i64,
    pub expiry_date: String,
    pub process_document_requirement: i64,
    pub process_id: String,
    pub process_name: String,
    pub status: String,
}
