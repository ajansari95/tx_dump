use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize)]
pub struct Pagination {
    pub next_key: Option<String>,
    pub total: String,
}

impl fmt::Display for Pagination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.next_key {
            Some(next_key) => write!(f, "Next Key: {}, Total: {}", next_key, self.total),
            None => write!(f, "Next Key: None, Total: {}", self.total)
        }
    }
}

// impl<'de> Deserialize<'de> for Pagination {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         struct Helper {
//             next_key: Option<String>,
//             total: String,
//         }
//
//         let helper = Helper::deserialize(deserializer)?;
//         let total = helper.total.parse::<String>().map_err(serde::de::Error::custom)?;
//
//         Ok(Pagination {
//             next_key: helper.next_key,
//             total,
//         })
//     }
// }
