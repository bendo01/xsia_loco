use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryDeserialization;

impl LibraryDeserialization {
    // --- Deserializers ---

    pub fn deserialize_date_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Accept null or "DD-MM-YYYY"
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) if !s.trim().is_empty() => NaiveDate::parse_from_str(&s, "%d-%m-%Y")
                .map(Some)
                .map_err(serde::de::Error::custom),
            _ => Ok(None),
        }
    }

    pub fn de_opt_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let v = Option::<Value>::deserialize(deserializer)?;
        match v {
            None | Some(Value::Null) => Ok(None),
            Some(Value::Number(n)) => n
                .as_u64()
                .map(|x| Some(x as u32))
                .ok_or_else(|| serde::de::Error::custom("expected unsigned integer")),
            Some(Value::String(s)) => {
                if s.trim().is_empty() {
                    Ok(None)
                } else {
                    s.parse::<u32>()
                        .map(Some)
                        .map_err(|e| serde::de::Error::custom(format!("invalid u32: {e}")))
                }
            }
            other => Err(serde::de::Error::custom(format!(
                "unexpected type for u32: {other:?}"
            ))),
        }
    }

    pub fn de_opt_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let v = Option::<Value>::deserialize(deserializer)?;
        match v {
            None => Ok(None),
            Some(Value::Null) => Ok(None),
            Some(Value::Number(n)) => n
                .as_f64()
                .map(|x| Some(x as f32))
                .ok_or_else(|| serde::de::Error::custom("expected float")),
            Some(Value::String(s)) => {
                if s.trim().is_empty() {
                    Ok(None)
                } else {
                    s.parse::<f32>()
                        .map(Some)
                        .map_err(|e| serde::de::Error::custom(format!("invalid f32: {e}")))
                }
            }
            other => Err(serde::de::Error::custom(format!(
                "unexpected type for f32: {other:?}"
            ))),
        }
    }
}