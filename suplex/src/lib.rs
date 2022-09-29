// use std::collections::HashMap;

use std::num::ParseIntError;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::header::HeaderMap;
use thiserror;

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

#[derive(Default, Debug)]
pub struct PlexUser {
    pub id: u64,
    pub uuid: String,
    pub email: String,
    // joined_at,
    pub username: String,
    pub title: String,
    // thumb,
    // hasPassword,
    pub auth_token: String,
    // subscription: Subscription
    // extra: Vec<PlexUserData>,
}

// #[derive(Serialize, Deserialize, Debug)]
// enum PlexUserData {
//     subscription(HashMap<String, String>),
//     roles(HashMap<String, String>),
//     entitlements(HashMap<String, String>),
//     profile_settings(HashMap<String, String>),
//     providers(HashMap<String, String>),
//     services(HashMap<String, String>),
//     username(String),
//     email(HashMap<String, String>),
//     joined_at(HashMap<String, String>),
//     #[serde(rename = "authentication-token")]
//     authentication_token(HashMap<String, String>),
// }

impl PlexUser {
    // PUBLIC -- log user in and create Self on success
    pub async fn authenticate<T>(username: T, password: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        apply_plex_product(&mut headers);
        apply_plex_version(&mut headers);
        apply_plex_client_identifier(&mut headers);

        let response = client
            .post("https://plex.tv/users/sign_in.xml")
            .headers(headers)
            .basic_auth(username.into(), Some(password.into()))
            .send()
            .await?;

        let text = response.text().await?;
        println!("{}", text);
        // TODO: just parse the damn XML by hand -- I can manually ignore shit I don't care about

        Self::_from_xml_text(text).await
    }

    // INNER -- create Self from XML text String
    async fn _from_xml_text(text: String) -> Result<Self> {
        let mut reader = Reader::from_str(text.as_str());
        let mut plex_user = PlexUser {
            ..Default::default()
        };
        loop {
            match reader.read_event() {
                Err(e) => return Err(SuplexError::XmlError { source: e }),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"user" => {
                        // id
                        if let Some(attr) = e.try_get_attribute("id")? {
                            plex_user.id = attr.unescape_value()?.to_string().parse()?;
                        }

                        // id
                        if let Some(attr) = e.try_get_attribute("uuid")? {
                            plex_user.uuid = attr.unescape_value()?.to_string();
                        }

                        // id
                        if let Some(attr) = e.try_get_attribute("email")? {
                            plex_user.email = attr.unescape_value()?.to_string();
                        }

                        // username
                        if let Some(attr) = e.try_get_attribute("username")? {
                            plex_user.username = attr.unescape_value()?.to_string();
                        }

                        // authToken
                        if let Some(attr) = e.try_get_attribute("authToken")? {
                            plex_user.auth_token = attr.unescape_value()?.to_string();
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Ok(plex_user)
    }
}

fn apply_plex_product(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Product", "TEST".parse().unwrap());
}

fn apply_plex_version(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Version", "TEST".parse().unwrap());
}

fn apply_plex_client_identifier(headers: &mut HeaderMap) {
    headers.insert("X-Plex-Client-Identifier", "12345abcdez".parse().unwrap());
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
