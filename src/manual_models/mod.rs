use serde::{Deserialize, Serialize};

pub mod historic_data_index;
pub mod historic_data_mark;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Resolution {
    #[serde(rename = "1m")]
    Variant1m,
    #[serde(rename = "5m")]
    Variant5m,
    #[serde(rename = "15m")]
    Variant15m,
    #[serde(rename = "30m")]
    Variant30m,
    #[serde(rename = "1h")]
    Variant1h,
    #[serde(rename = "1d")]
    Variant1d,
    #[serde(rename = "1w")]
    Variant1w,
}

impl Default for Resolution {
    fn default() -> Resolution {
        Self::Variant1m
    }
}
