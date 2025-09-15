use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Deserialize optional date in `DD-MM-YYYY` format or `null`.
pub fn deserialize_date_opt<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.trim().is_empty() => NaiveDate::parse_from_str(&s, "%d-%m-%Y")
            .map(Some)
            .map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

/// Deserialize optional `u32` from number, string, or null.
pub fn de_opt_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Value>::deserialize(deserializer)?;
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_u64()
            .map(|x| Some(x as u32))
            .ok_or_else(|| serde::de::Error::custom("expected unsigned integer")),
        Some(Value::String(s)) if s.trim().is_empty() => Ok(None),
        Some(Value::String(s)) => s
            .parse::<u32>()
            .map(Some)
            .map_err(|e| serde::de::Error::custom(format!("invalid u32: {e}"))),
        other => Err(serde::de::Error::custom(format!(
            "unexpected type for u32: {other:?}"
        ))),
    }
}

pub fn de_opt_i32<'de, D>(de: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    Ok(Some(match serde_json::Value::deserialize(de)? {
        serde_json::Value::Number(n) => n
            .as_i64()
            .ok_or_else(|| Error::invalid_type(Unexpected::Other("number"), &"i32"))?
            as i32,
        serde_json::Value::String(s) => s.trim().parse::<i32>().map_err(|_| {
            Error::invalid_value(Unexpected::Str(&s), &"numeric string for i32")
        })?,
        serde_json::Value::Null => return Ok(None),
        v => return Err(Error::invalid_type(Unexpected::Other(v.to_string().as_str()), &"i32 or numeric string")),
    }))
}

/// Deserialize optional `f32` from number, string, or null.
pub fn de_opt_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Value>::deserialize(deserializer)?;
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_f64()
            .map(|x| Some(x as f32))
            .ok_or_else(|| serde::de::Error::custom("expected float")),
        Some(Value::String(s)) if s.trim().is_empty() => Ok(None),
        Some(Value::String(s)) => s
            .parse::<f32>()
            .map(Some)
            .map_err(|e| serde::de::Error::custom(format!("invalid f32: {e}"))),
        other => Err(serde::de::Error::custom(format!(
            "unexpected type for f32: {other:?}"
        ))),
    }
}
