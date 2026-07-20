use serde::{Deserialize, Serializer};

pub mod time;

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: u8 = u8::deserialize(deserializer)?;
    println!("value: {}", value);
    match value {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::custom(format!(
            "invalid value: {}, expected 0 or 1",
            other
        ))),
    }
}
pub fn bool_as_int<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(if *value { 1 } else { 0 })
}
