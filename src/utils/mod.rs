use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

pub fn arbitrary_tx_amount<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Amount<T> {
        Number(T),
        String(String),
    }

    match Amount::<T>::deserialize(deserializer)? {
        Amount::Number(i) => Ok(i),
        Amount::String(s) if s.is_empty() => Ok(T::default()),
        Amount::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
    }
}
