use iced::settings::Settings;
use suplex;

use anyhow::Result;
use iced::pure::{column, container, text, Application, Element};
use iced::Length;
use iced::{executor, Command};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct LoginInfo {
    username: Option<String>,
    password: Option<String>,
    auth_token: Option<String>,
}

#[derive(Debug)]
enum SimPlexMessage {}

enum AppState {
    Login,
    MainView,
}

struct SimPlexApp {
    state: AppState,
    user_info: LoginInfo,
}

impl Application for SimPlexApp {
    type Executor = executor::Default;
    type Message = SimPlexMessage;
    type Flags = LoginInfo;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::Login,
                user_info: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SimPlex Tool")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {}
        Command::none()
    }

    fn view(self: &SimPlexApp) -> Element<Self::Message> {
        // TODO: match app state
        // TODO: create login info in case there's no auth token and no username/password saved
        // TODO: save username (not password) to disk, along with auth token if we got one
        // TODO: if there's an auth token, try to get some info
        // TODO: as soon as you get some good info, switch to the MainView state
        let view = column().push(text(format!(
            "Hi, {}",
            self.user_info.username.as_ref().unwrap()
        )));

        container(view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

pub fn main() -> Result<()> {
    println!("I am SimPlex!");

    let login_file_data = fs::read_to_string("login_info.json")?;
    let mut login_info: LoginInfo = from_str(&login_file_data)?;

    SimPlexApp::run(Settings::with_flags(login_info))?;

    // let client =
    //     suplex::PlexUser::authenticate(&login_info.username?, &login_info.password?).await?;

    // login_info.auth_token = Some(client.auth_token);
    // println!("Logged in as {}", client.username);

    // fs::write("login_info.json", to_string(&login_info)?)?;
    Ok(())
}
