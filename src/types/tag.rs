use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HassTag {
    pub id: String,
    pub last_scanned: DateTime<Utc>,
    pub device_id: String,
    pub name: String,
}
