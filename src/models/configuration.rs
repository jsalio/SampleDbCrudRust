use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub handle: Option<i64>,
    pub id: i64,
    pub is_required: bool,
    pub is_template: bool,
    pub name: String,
    pub status: String,
}