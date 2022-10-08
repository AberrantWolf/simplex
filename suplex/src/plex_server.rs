use quick_xml::{events::Event, Reader};

use reqwest::header::HeaderMap;
use xml_struct_derive::XmlStruct;

use xml_struct::XmlStruct;
use xml_struct::XmlStructError;

use crate::util::apply_base_headers;
use crate::util::apply_plex_user_token;
use crate::Result;

#[derive(Default, Debug, Clone, XmlStruct)]
#[xmlElement = b"server"]
pub struct PlexServer {
    first_field: String,
    another_field: u64,
}

impl PlexServer {
    pub async fn query_servers(token: String) -> Result<()> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        apply_base_headers(&mut headers);
        apply_plex_user_token(&mut headers, token.as_str());

        println!("{:?}", headers);

        let response = client
            .get("https://plex.tv/api/resources?includeHttps=1&includeRelay=1")
            .headers(headers)
            .send()
            .await?;

        print!("{}", response.text().await?);

        Ok(())
    }
}
