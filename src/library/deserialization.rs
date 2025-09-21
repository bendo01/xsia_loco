use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use serde::de::{Error as DeError, Unexpected};
use serde_json::Value;

/// Option<i32> from number | numeric string | null
pub fn de_opt_i32<'de, D>(de: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<Value> = Option::<Value>::deserialize(de)?;
    Ok(match v {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => {
            let i = n.as_i64().ok_or_else(|| DeError::invalid_type(Unexpected::Other("number"), &"i32"))?;
            Some(i32::try_from(i).map_err(|_| DeError::custom("i32 out of range"))?)
        }
        Some(Value::String(s)) => {
            let s = s.trim();
            if s.is_empty() { None } else { Some(s.parse::<i32>().map_err(|_| DeError::invalid_value(Unexpected::Str(s), &"numeric string for i32"))?) }
        }
        Some(other) => return Err(DeError::custom(format!("unexpected type for i32: {other:?}"))),
    })
}

/// Option<u32> from number | numeric string | null
pub fn de_opt_u32<'de, D>(de: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<Value> = Option::<Value>::deserialize(de)?;
    Ok(match v {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => {
            let u = n.as_u64().ok_or_else(|| DeError::invalid_type(Unexpected::Other("number"), &"u32"))?;
            Some(u32::try_from(u).map_err(|_| DeError::custom("u32 out of range"))?)
        }
        Some(Value::String(s)) => {
            let s = s.trim();
            if s.is_empty() { None } else { Some(s.parse::<u32>().map_err(|e| DeError::custom(format!("invalid u32: {e}")))? ) }
        }
        Some(other) => return Err(DeError::custom(format!("unexpected type for u32: {other:?}"))),
    })
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


// ---------- helpers ----------
pub fn de_opt_string_or_int<'de, D>(d: D) -> Result<Option<String>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrIntNull { S(String), I(i64), F(f64), N(serde_json::Value) }
    match Option::<StrIntNull>::deserialize(d)? {
        Some(StrIntNull::S(s)) => Ok(Some(s)),
        Some(StrIntNull::I(i)) => Ok(Some(i.to_string())),
        Some(StrIntNull::F(f)) => Ok(Some({
            // hilangkan .0 kalau bilangan bulat
            if f.fract() == 0.0 { (f as i64).to_string() } else { f.to_string() }
        })),
        Some(StrIntNull::N(v)) if v.is_null() => Ok(None),
        Some(_) => Err(D::Error::invalid_type(Unexpected::Other("non str/int"), &"str/int/null")),
        None => Ok(None),
    }
}

pub fn de_opt_boolish<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum B { B(bool), I(i64), S(String), N(serde_json::Value) }
    Ok(match Option::<B>::deserialize(d)? {
        None => None,
        Some(B::N(v)) if v.is_null() => None,
        Some(B::N(_)) => None,
        Some(B::B(b)) => Some(b),
        Some(B::I(i)) => Some(i != 0),
        Some(B::S(s)) => {
            let s = s.trim().to_ascii_lowercase();
            Some(matches!(s.as_str(), "1" | "true" | "ya" | "y" | "yes"))
        }
    })
}

/// Deserialize optional date in `DD-MM-YYYY` format or `null`.
pub fn de_opt_date_dmy<'de, D>(d: D) -> Result<Option<NaiveDate>, D::Error>
where D: Deserializer<'de> {
    let s: Option<String> = Option::deserialize(d)?;
    if let Some(s) = s {
        // FEEDER sering kirim dd-MM-yyyy
        NaiveDate::parse_from_str(&s, "%d-%m-%Y")
            .map(Some)
            .or_else(|_| NaiveDate::parse_from_str(&s, "%Y-%m-%d").map(Some))
            .map_err(D::Error::custom)
    } else {
        Ok(None)
    }
}