use {
    serde::{
        Serialize,
        Serializer,
        Deserialize,
        Deserializer,
        de::{
            self,
            Visitor,
        }
    },
    std::fmt,
    anyhow::{
        anyhow,
        Result
    },
};

/// Blizzard should no longer be used, since most requests with it will fail!
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MembershipType {
    None,
    Xbox,
    Psn,
    Steam,
    Blizzard,
    Stadia,
    BungieNet,
    All,
}

impl MembershipType {
    pub fn into_i32(&self) -> i32 {
        match self {
            MembershipType::None => 0,
            MembershipType::Xbox => 1,
            MembershipType::Psn => 2,
            MembershipType::Steam => 3,
            MembershipType::Blizzard => 4,
            MembershipType::Stadia => 5,
            MembershipType::BungieNet => 254,
            MembershipType::All => -1,
        }
    }

    pub fn from_i32(v: i32) -> Result<Self> {
        Ok(match v {
            0 => MembershipType::None,
            1 => MembershipType::Xbox,
            2 => MembershipType::Psn,
            3 => MembershipType::Steam,
            4 => MembershipType::Blizzard,
            5 => MembershipType::Stadia,
            254 => MembershipType::BungieNet,
            -1 => MembershipType::All,
            _ => return Err(anyhow!("the value {} is not a valid value for MembershipType", v)),
        })
    }
}

impl Serialize for MembershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_i32(self.into_i32())
    }
}

struct MembershipTypeVisitor;

impl<'de> Visitor<'de> for MembershipTypeVisitor {
    type Value = MembershipType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i32 representing a valid MembershipType")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error
    {
        Ok(match MembershipType::from_i32(value as i32) {
            Ok(x) => x,
            Err(e) => return Err(E::custom(e)),
        })
    }
}

impl<'de> Deserialize<'de> for MembershipType {
    fn deserialize<D>(d: D) -> Result<MembershipType, D::Error>
    where
        D: Deserializer<'de>
    {
        d.deserialize_i32(MembershipTypeVisitor)
    }
}