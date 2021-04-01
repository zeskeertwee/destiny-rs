/*
    rust types for the types described in the API docs
*/

pub(crate) use chrono::{
    NaiveDateTime,
    DateTime,
    Utc
};

use std::{fmt, marker::PhantomData};
use {
    serde::{
        de::{
            self,
            Visitor,
            MapAccess,
            Error,
        },
        Deserialize
    },
    std::{
        collections::HashMap,
        str::FromStr,
        hash,
    }
};

pub type Int32 = i32;
pub type Int64 = i64;
pub type Uint32 = u32;
pub type Uint64 = u64;
pub type Hash = Uint32;

pub type APIdateTime = DateTime::<Utc>;

#[derive(Debug, Deserialize)]
pub struct APIdateRange {
    #[serde(deserialize_with = "from_timestamp")]
    pub start: APIdateTime,
    #[serde(deserialize_with = "from_timestamp")]
    pub end: APIdateTime
}

pub(crate) fn serde_none<T>() -> Option<T> {
    None
}

pub(crate) fn serde_empty_vec<T>() -> Vec<T> {
    Vec::new()
}

pub(crate) fn serde_empty_map<K, V>() -> HashMap<K, V> {
    HashMap::new()
}

struct APIdateTimeDecoder;

// https://stackoverflow.com/questions/57614558/how-to-use-a-custom-serde-deserializer-for-chrono-timestamps
impl<'de> Visitor<'de> for APIdateTimeDecoder {
    type Value = DateTime::<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string representation of datetime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ") {
            Ok(t) => {
                Ok(DateTime::from_utc(t, Utc))
            },
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }
}

pub(crate) fn from_timestamp<'de, D>(d: D) -> Result<DateTime::<Utc>, D::Error>
where
    D: de::Deserializer<'de>
{
    d.deserialize_string(APIdateTimeDecoder)
}

pub(crate) fn from_timestamp_nullable<'de, D>(d: D) -> Result<Option<DateTime::<Utc>>, D::Error>
where
    D: de::Deserializer<'de>
{
    Ok(match d.deserialize_str(APIdateTimeDecoder) {
        Ok(x) => Some(x),
        Err(_) => None,
    })
}

struct RawStringVisitor;

impl<'de> Visitor<'de> for RawStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E> 
    where
        E: de::Error,
    {
        Ok(s)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(s.to_string())
    }
}

pub(crate) fn int64_from_str<'de, D>(d: D) -> Result<Int64, D::Error>
where
    D: de::Deserializer<'de>
{
    let s = d.deserialize_string(RawStringVisitor)?;
    match s.parse::<i64>() {
        Ok(x) => Ok(x),
        Err(_) => Err(de::Error::custom("invalid type for i64 deserialization")),
    }
}

pub(crate) fn int64_from_str_nullable<'de, D>(d: D) -> Result<Option<Int64>, D::Error>
where
    D: de::Deserializer<'de>
{
    let s = d.deserialize_string(RawStringVisitor)?;
    match s.parse::<i64>() {
        Ok(x) => Ok(Some(x)),
        Err(_) => Ok(None),
    }
}

pub(crate) fn uint32_from_str<'de, D>(d: D) -> Result<Uint32, D::Error>
where
    D: de::Deserializer<'de>
{
    let s = d.deserialize_string(RawStringVisitor)?;
    match s.parse::<u32>() {
        Ok(x) => Ok(x),
        Err(_) => Err(de::Error::custom("invalid type for i64 deserialization")),
    }
}

struct StringMapVisitor<K, V> {
    key: PhantomData<K>,
    value: PhantomData<V>
}

impl<'de, K, V> Visitor<'de> for StringMapVisitor<K, V>
where 
    K: hash::Hash + Eq + FromStr,
    V: Deserialize<'de>,
{
    type Value = HashMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map that contains uint32's as strings")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>
    {
        let mut result = HashMap::new();
        
        while let Some((key, value)) = map.next_entry::<String, V>()? {
            result.insert(match key.parse::<K>() {
                Ok(x) => x,
                Err(_) => return Err(A::Error::custom("invalid value"))
            }, value);
        }

        Ok(result)
    }
}

pub(crate) fn uint32_map_from_str<'de, D, V>(d: D) -> Result<HashMap<Hash, V>, D::Error>
where
    D: de::Deserializer<'de>,
    V: Deserialize<'de>
{
    d.deserialize_map(StringMapVisitor { key: PhantomData, value: PhantomData })
}