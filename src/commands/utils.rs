use miniscript::bitcoin::{self, consensus, hashes::hex::FromHex};
use serde::{de, Deserialize, Deserializer, Serializer};

/// Serialize an amount as sats
pub fn ser_amount<S: Serializer>(amount: &bitcoin::Amount, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(amount.to_sat())
}

/// Deserialize an amount from sats
pub fn deser_amount_from_sats<'de, D>(deserializer: D) -> Result<bitcoin::Amount, D::Error>
where
    D: Deserializer<'de>,
{
    let a = u64::deserialize(deserializer)?;
    Ok(bitcoin::Amount::from_sat(a))
}

pub fn to_base64_string<T: consensus::Encodable>(t: T) -> String {
    base64::encode(consensus::serialize(&t))
}

pub fn ser_base64<S, T>(t: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: consensus::Encodable,
{
    s.serialize_str(&to_base64_string(t))
}

pub fn deser_base64<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: consensus::Decodable,
{
    let s = String::deserialize(d)?;
    let s = base64::decode(s).map_err(de::Error::custom)?;
    consensus::deserialize(&s).map_err(de::Error::custom)
}

pub fn ser_hex<S, T>(t: T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: consensus::Encodable,
{
    s.serialize_str(&consensus::encode::serialize_hex(&t))
}

pub fn deser_hex<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: consensus::Decodable,
{
    let s = String::deserialize(d)?;
    let s = Vec::from_hex(&s).map_err(de::Error::custom)?;
    consensus::deserialize(&s).map_err(de::Error::custom)
}
