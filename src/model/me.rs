use chrono::{DateTime, NaiveDate, Utc};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RecurringParameter {
    pub custom_period: Option<i64>,
    pub estimated_seconds: i64,
    pub parameter_end_date: Option<NaiveDate>,
    pub parameter_start_date: NaiveDate,
    pub period: String,
    pub project_start_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentPeriod {
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: u32,
    pub workspace_id: u32,
    pub client_id: Option<u32>,
    pub name: String,
    pub is_private: bool,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
    pub color: String,
    pub billable: Option<bool>,
    pub template: Option<bool>,
    pub auto_estimates: Option<bool>,
    pub estimated_hours: Option<u32>,
    pub rate: Option<f32>,
    pub currency: Option<String>,
    pub recurring: bool,
    pub recurring_parameters: Option<Vec<RecurringParameter>>,
    pub current_period: Option<CurrentPeriod>,
    pub fixed_fee: Option<f32>,
    pub actual_hours: Option<u32>,
    pub end_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub first_time_entry: Option<String>,
    pub rate_last_updated: Option<DateTime<Utc>>,
    // deprecated: pub wid: u32,
    // deprecated: pub cid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub id: u32,
    pub name: String,
    pub workspace_id: u32,
}
