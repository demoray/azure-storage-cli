use azure_core::request_options::Metadata;
use azure_storage_blobs::prelude::*;
use std::{error::Error, result::Result, str::FromStr};

/// Parse a single key-value pair of `X=Y` into a typed tuple of `(X, Y)`.
///
/// # Errors
/// Returns an `Err` if any of the keys or values cannot be parsed or if no `=` is found.
pub fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    if let Some((key, value)) = s.split_once('=') {
        Ok((key.parse()?, value.parse()?))
    } else {
        Err(format!("invalid KEY=value: no `=` found in `{s}`").into())
    }
}

pub(crate) fn to_tags(value: Vec<(String, String)>) -> Tags {
    let mut tags = Tags::new();
    for (key, value) in value {
        tags.insert(key, value);
    }
    tags
}

pub(crate) fn to_metadata(value: Vec<(String, String)>) -> Metadata {
    let mut metadata = Metadata::new();
    for (key, value) in value {
        metadata.insert(key, value);
    }
    metadata
}

pub(crate) fn round_up(x: u64, y: u64) -> u64 {
    (x / y + u64::from(x % y != 0)) * y
}
