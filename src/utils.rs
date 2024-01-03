use azure_core::{
    date::parse_rfc3339,
    error::{Error, ErrorKind},
    request_options::Metadata,
};
use azure_storage_blobs::prelude::Tags;
use azure_storage_datalake::Properties;
use clap::ValueEnum;
use std::{error::Error as StdError, ops::Add, result::Result, str::FromStr};
use time::OffsetDateTime;

/// Parse a single key-value pair of `X=Y` into a typed tuple of `(X, Y)`.
///
/// # Errors
/// Returns an `Err` if any of the keys or values cannot be parsed or if no `=` is found.
pub fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn StdError + Send + Sync + 'static>>
where
    T: FromStr,
    T::Err: StdError + Send + Sync + 'static,
    U: FromStr,
    U::Err: StdError + Send + Sync + 'static,
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

pub(crate) fn to_properties(value: Vec<(String, String)>) -> Properties {
    let mut properties = Properties::new();
    for (key, value) in value {
        properties.insert(key, value);
    }
    properties
}

pub(crate) fn round_up(x: u64, y: u64) -> u64 {
    (x / y + u64::from(x % y != 0)) * y
}

pub(crate) fn parse_time(s: &str, format: TimeFormat) -> azure_core::Result<OffsetDateTime> {
    match format {
        TimeFormat::Rfc3339 => parse_rfc3339(s),
        TimeFormat::Offset => {
            let duration: time::Duration = parse_duration::parse(s)
                .map_err(|e| Error::new(ErrorKind::DataConversion, e))?
                .try_into()
                .map_err(|e| Error::new(ErrorKind::DataConversion, e))?;

            let now = OffsetDateTime::now_utc();
            Ok(now.add(duration))
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimeFormat {
    /// Specific date and time, as described in <https://www.rfc-editor.org/rfc/rfc3339>.
    /// Examples include `1999-09-10T21:59:22Z` and `1999-09-10T03:05:07.3845533+01:00`
    Rfc3339,
    /// Offset from `now`, as parsed by <https://docs.rs/parse_duration/latest/parse_duration/>
    /// Examples include `10d`, `1h`, `1h30m`, and `1h30m10s`
    Offset,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Protocol {
    Https,
    HttpHttps,
}
