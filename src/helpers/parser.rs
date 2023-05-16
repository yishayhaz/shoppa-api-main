use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};
use serde_json;

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

    // let s = deserializer.deserialize_str(visitor)

    // let v: Vec<T> = serde_json::from_str(s.as_str()).map_err(de::Error::custom)?;

    // Ok(v)
    Ok(vec![])
}

// #[serde(default, deserialize_with = "empty_string_as_none")]
