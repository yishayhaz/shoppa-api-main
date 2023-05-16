use serde::{de, Deserialize, Deserializer};
use serde_json;
use std::{fmt, str::FromStr};

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn deserialize_query_array<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    T::Err: fmt::Display,
{
    let s = String::deserialize(deserializer)?;

    let v: Vec<String> = serde_json::from_str(&s).map_err(de::Error::custom)?;

    let mut result = Vec::with_capacity(v.len());

    for s in v {
        result.push(T::from_str(&s).map_err(|e| de::Error::custom(format!("{}: {}", e, s)))?);
    }

    Ok(result)
}
