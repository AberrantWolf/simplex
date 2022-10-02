//!
//! Suplex `PlexUser`
//!
use crate::util::{
    apply_plex_client_identifier, apply_plex_product, apply_plex_version, Result, REGEX_SET,
};
use crate::{build_plex_user_add_fn, xml_match_attrs, SuplexError};

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex::Regex;
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
                //if let Ok(Self::build_with_regex(&text)){ // Absolutely not faster :(
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

    // This will build an add method, which add's &str to a PlexUser's appropriate field by
    // matching substrings.
    build_plex_user_add_fn!(
        "email", email, "username", username, "title", title, "en", auth_token, "uuid", uuid
    );

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
                Ok(Event::Start(e)) => {
                    xml_match_attrs!(plex_user, e, "id", "uuid", "username", "authToken")
                }
                _ => (),
            }
        }
        Ok(plex_user)
    }
    // INNER -- create Self from XML text String
    /// Builds a [`PlexUser`] using Regexes and String-matching, rather than the
    /// [`from_xml_text()`]'s line-by-line approach.
    pub fn _build_with_regex(xml: &str) -> Result<Self> {
        let regexes: Vec<_> = REGEX_SET
            .patterns()
            // .into_par_iter() // t1.elapsed().as_nanos() = 2459129
            .iter() // t1.elapsed().as_nanos() = 1602265
            .map(|pat| Regex::new(pat).unwrap())
            .collect();

        let mut pu = PlexUser {
            ..Default::default()
        };

        REGEX_SET
            .matches(xml)
            .iter()
            .map(|match_idx| &regexes[match_idx])
            .flat_map(|pat| pat.find(xml))
            .for_each(|res| {
                _ = pu.add(res.as_str());
            });
        Ok(pu)
    }
}
