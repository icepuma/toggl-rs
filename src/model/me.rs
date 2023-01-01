use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Me {
    pub api_token: Option<String>,
    pub at: DateTime<Utc>,
    pub beginning_of_week: u8,
    pub country_id: u32,
    pub created_at: DateTime<Utc>,
    pub default_workspace_id: u32,
    pub email: String,
    pub fullname: String,
    pub id: u32,
    pub image_url: String,
    pub intercom_hash: Option<String>,
    pub openid_email: Option<String>,
    pub openid_enabled: bool,
    pub options: Option<Value>,
    pub timezone: String,
    pub updated_at: DateTime<Utc>,
}
