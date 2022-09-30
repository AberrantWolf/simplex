use lazy_static::lazy_static;
use regex::Error as RegXError;
use regex::RegexSet;
use reqwest::header::HeaderMap;
use std::num::ParseIntError;
use thiserror;

#[macro_export]
/// Expands all the potential xml<attribute> tags into a collection of `if-let`s to parse strings
/// and assign field values to [`PlexUser`]s.
macro_rules! xml_match_attrs {
    ($plex_user:expr, $e:expr, $($tag:expr),*) => {{
        if let b"user" = $e.name().as_ref() {
            $( if let Some(attr) = $e.try_get_attribute($tag)? {
                $plex_user.id = attr.unescape_value()?.to_string().parse()?;
            })*
        }
    }};
    }

/// Generates the code for the [`PlexUser::Add`] method.
/// Uses regex matches and string matching to populate the correct field for the provided &str.
#[macro_export]
macro_rules! build_plex_user_add_fn {
    ($($tag:expr, $field:ident),*) => {
    /// Adds a field value to the right spot in a [`PlexUser`] my matching contents of a &str.
    /// powered by regexes.
    fn add(&mut self, reg_match: &str) -> Result<()> {
            $(if reg_match.contains($tag) {
                if let Some(v) = reg_match.split('=').last() {
                    self.$field= v.trim().trim_matches('"').to_string();
                }
            })*
            // This one's irregular, so it gets hardcoded.
            if reg_match.contains("id") && !reg_match.contains("uu"){
                if let Some(v) = reg_match.split('=').last() {
                    self.id = v.trim().trim_matches('"').parse::<u64>()?;
                }
            }
            Ok(())
            }
        };

}
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
    #[error("Regex Error, syntax or size related no doubt...")]
    RegexError {
        #[from]
        source: RegXError,
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

lazy_static! {
    pub(crate) static ref REGEX_SET: RegexSet =
        RegexSet::new(&[
            r#"(email="[a-z]+@[a-z]+.[a-z]+)"#,
            r#"(id="\d+")"#,
            r#"(uuid="\w+")"#,
            r#"(username="\w+")"#,
            r#"(title="\w+")"#,
            r#"(authToken="\w+")"#, //NOTE: there's many.. variants of this?
        ]).expect("Error compiling regexes, probably you've made a syntax error. ");
}
