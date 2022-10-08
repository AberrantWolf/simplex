use reqwest::header::HeaderMap;
use std::num::ParseIntError;
use thiserror;

#[macro_export]
/// Implements clone for a type you pass to it.
macro_rules! impl_clone_for{
    ($($t:ty),*) =>
    {
        $(impl Copy for $t{})*

        $(impl Clone for $t{
            fn clone(&self) -> Self{
                *self
            }
        })*
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SuplexError {
    // Ours:
    #[error("Failed to update\ncode:{code}\nmsg:{msg}")]
    UpdateFailure { code: u16, msg: String },

    #[error("Failed to fetch data \ncode:{code}\nmsg:{msg}")]
    FetchFailure { code: u16, msg: String },

    #[error("The requested resource could not be found,\nmsg:{msg}")]
    NotFound { code: u16, msg: String },

    // Exporting errors
    #[error("unable to deserialize a response")]
    SerdeDeserializeError {
        #[from]
        source: quick_xml::DeError,
    },
    #[error("error making an HTTP request")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },
    #[error("error reading XML data")]
    XmlError {
        #[from]
        source: quick_xml::Error,
    },
    #[error("error converting string to number")]
    ParseError {
        #[from]
        source: ParseIntError,
    },
}

pub type Result<T> = std::result::Result<T, SuplexError>;

pub(crate) fn apply_plex_platform(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Platform", "TEST".parse().unwrap());
}

pub(crate) fn apply_plex_platform_version(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Platform-Version", "TEST".parse().unwrap());
}

pub(crate) fn apply_plex_provides(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Provides", "controller".parse().unwrap());
}

pub(crate) fn apply_plex_product(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Product", "TEST".parse().unwrap());
}

pub(crate) fn apply_plex_version(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Version", "0.0.1".parse().unwrap());
}

pub(crate) fn apply_plex_device(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Device", "Undefined".parse().unwrap());
}

pub(crate) fn apply_plex_device_name(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Device-Name", "Undefined".parse().unwrap());
}

pub(crate) fn apply_plex_client_identifier(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Client-Identifier", "12345abcdez".parse().unwrap());
}

pub(crate) fn apply_plex_client_sync_version(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Sync-Version", "2".parse().unwrap());
}

pub(crate) fn apply_plex_client_features(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Features", "external-media".parse().unwrap());
}

pub(crate) fn apply_plex_user_token(headers: &mut HeaderMap, token: &str) {
    headers.insert("X-Plex-Token", token.parse().unwrap());
}

pub(crate) fn apply_base_headers(headers: &mut HeaderMap) {
    apply_plex_platform(headers);
    apply_plex_platform_version(headers);
    apply_plex_provides(headers);
    apply_plex_product(headers);
    apply_plex_version(headers);
    apply_plex_device(headers);
    apply_plex_device_name(headers);
    apply_plex_client_identifier(headers);
    apply_plex_client_sync_version(headers);
    apply_plex_client_features(headers);
}
