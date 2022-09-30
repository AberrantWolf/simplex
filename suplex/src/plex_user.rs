use crate::util::{
    apply_plex_client_identifier, apply_plex_product, apply_plex_version, extract_xml_attr, Result,
};
use crate::SuplexError;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::header::HeaderMap;

#[derive(Default, Debug, Clone)]
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
}

impl PlexUser {
    // PUBLIC -- log user in and create Self on success
    pub async fn authenticate<T>(username: T, password: T) -> Option<Self>
    where
        T: Into<String> + std::fmt::Display,
    {
        let result = Self::_try_login(&username, &password).await;
        if let Ok(text) = result {
            println!("{}", text);
            if let Ok(result) = Self::_from_xml_text(text).await {
                Some(result)
            } else {
                // TODO: Log the error...?
                None
            }
        } else {
            // TODO: Log the error...?
            None
        }
    }

    async fn _try_login<T>(username: &T, password: &T) -> Result<String>
    where
        T: Into<String> + std::fmt::Display,
    {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        apply_plex_product(&mut headers);
        apply_plex_version(&mut headers);
        apply_plex_client_identifier(&mut headers);

        let response = client
            .post("https://plex.tv/users/sign_in.xml")
            .headers(headers)
            .basic_auth(username.to_owned(), Some(password.to_owned()))
            .send()
            .await?;

        Ok(response.text().await?)
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
                        extract_xml_attr!(id, e, plex_user);

                        // uuid
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
                        // TODO: The current macro won't handle this because the two are different
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
