use reqwest::header::HeaderMap;
use std::num::ParseIntError;
use thiserror;

macro_rules! extract_xml_attr {
    ($attrib_name:ident, $e:ident, $plex_user:ident) => {
        if let Some(attr) = $e.try_get_attribute(String::from(stringify!($attrib_name)))? {
            $plex_user.$attrib_name = attr.unescape_value()?.to_string().parse()?;
        }
    };
}

pub(crate) use extract_xml_attr;

#[derive(thiserror::Error, Debug)]
pub enum SuplexError {
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

pub(crate) fn apply_plex_product(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Product", "TEST".parse().unwrap());
}

pub(crate) fn apply_plex_version(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Version", "TEST".parse().unwrap());
}

pub(crate) fn apply_plex_client_identifier(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Client-Identifier", "12345abcdez".parse().unwrap());
}
