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

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub archived: bool,
    pub at: DateTime<Utc>,
    pub id: u32,
    pub name: String,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub wid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub feature_id: u32,
    pub name: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Features {
    pub workspace_id: u32,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub city: String,
    pub city_lat_long: String,
    pub state: String,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrialInfo {
    pub trial: bool,
    pub trial_available: bool,
    pub trial_end_date: Option<DateTime<Utc>>,
    pub next_payment_date: Option<DateTime<Utc>>,
    pub last_pricing_plan_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub admin: bool,
    pub at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub id: u32,
    pub is_chargify: bool,
    pub is_multi_workspace_enabled: bool,
    pub is_unified: bool,
    pub max_workspaces: u32,
    pub name: String,
    pub owner: bool,
    pub payment_methods: Option<String>,
    pub pricing_plan_id: u32,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub trial_info: TrialInfo,
    pub user_count: u32,
}
