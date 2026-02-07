use serde::{Deserialize, Serialize};

/// Country information returned by region detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    pub country_code: String,
    pub country_name: String,
    pub supported: bool,
}
