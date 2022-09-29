use suplex;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
    auth_token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("I am SimPlex!");

    let login_file_data = fs::read_to_string("login_info.json")?;
    let mut login_info: LoginInfo = from_str(&login_file_data)?;

    let client = suplex::PlexUser::authenticate(&login_info.username, &login_info.password).await?;

    login_info.auth_token = Some(client.auth_token);
    println!("Logged in as {}", client.username);

    fs::write("login_info.json", to_string(&login_info)?)?;

    Ok(())
}
