use serde::Deserialize;
use std::fmt;

struct ClientConfig {
    base_url: String,
    api_key: Option<String>,
    // ... other configuration parameters
}
impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.rustmaps.com/v4".to_string(),
            api_key: None,
        }
    }
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Meta {
    status: String,
    status_code: u64,
    errors: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug)]
pub struct LimitsDataTulpe {
    current: u64,
    allowed: u64,
}

#[derive(Deserialize, Default, Debug)]
pub struct LimitsData {
    concurrent: LimitsDataTulpe,
    monthly: LimitsDataTulpe,
}

#[derive(Deserialize, Default, Debug)]
#[allow(unused)]
pub struct Limits {
    #[serde(default)]
    meta: Meta,
    #[serde(default)]
    data: LimitsData,
}

impl fmt::Display for Limits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Concurrent Limits: {}/{}; Monthly Limits: {}/{}",
            self.data.concurrent.current,
            self.data.concurrent.allowed,
            self.data.monthly.current,
            self.data.monthly.allowed
        )
    }
}
