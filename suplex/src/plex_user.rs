//!
//! Suplex `PlexUser`
//!
use crate::util::{apply_plex_client_identifier, apply_plex_product, apply_plex_version, Result};

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::header::HeaderMap;
use xml_struct::{XmlStruct, XmlStructError};
use xml_struct_derive::*;

#[derive(Default, Debug, Clone, XmlStruct)]
#[xmlElement = "user"]
pub struct PlexUser {
    pub id: u64,
    pub uuid: String,
    pub email: String,
    // joined_at,
    pub username: String,
    pub title: String,
    pub thumb: String,
    // hasPassword,
    #[xmlName = "authToken"]
    pub auth_token: String,
    // subscription: Subscription
}

impl PlexUser {
    pub async fn authenticate<T>(username: T, password: T) -> Option<Self>
    where
        T: Into<String> + std::fmt::Display,
    {
        let result = Self::_try_login(&username, &password).await;
        if let Ok(text) = result {
            println!("{}", text);
            if let Ok(result) = Self::from_xml(text) {
                Some(result)
            } else {
                // TODO: Log the error...?
                // NOTE: I actually think custom result types would be nicer here, see the
                // additions to SuplexError I added, you'd construct them like this:
                //  Err(SuplexError::FetchFailure {
                //      code: 400,
                //      msg: "bad request".to_string(),
                //  });
                // which would let you have the codes you mentioned in a nice way maybe?
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
}
