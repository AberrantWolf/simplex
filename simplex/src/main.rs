use anyhow::Result;
use iced::pure::{button, column, container, row, text, text_input, Application, Element};
use iced::settings::Settings;
use iced::{executor, Alignment, Command, Length};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;
use suplex::plex_server::PlexServer;

use suplex::plex_user::PlexUser;

#[derive(Debug, Serialize, Deserialize)]
struct LoginInfo {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    auth_token: Option<String>,
}

#[derive(Debug, Clone)]
enum SimPlexMessage {
    Pass,
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    DoLogIn,
    LoginResult(Option<PlexUser>),
}

enum AppState {
    Login,
    MainView(PlexUser),
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
        if let None = flags.auth_token {
            // Go to a "logging in" state
            // Use https://plex.tv/users/account to get account info (and verify the token is valid)
            (
                Self {
                    state: AppState::Login,
                    user_info: flags,
                },
                Command::none(),
            )
        } else {
            let user_info = PlexUser {
                username: flags.username.clone(),
                auth_token: flags.auth_token.as_ref().unwrap().clone(),
                ..Default::default()
            };
            let token = user_info.auth_token.clone();
            (
                Self {
                    state: AppState::MainView(user_info),
                    user_info: flags,
                },
                Command::perform(PlexServer::query_servers(token), |_| SimPlexMessage::Pass),
            )
        }
    }

    fn title(&self) -> String {
        String::from("SimPlex Tool")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            SimPlexMessage::Pass => Command::none(),
            SimPlexMessage::LoginUsernameChanged(s) => {
                self.user_info.username = s;
                Command::none()
            }
            SimPlexMessage::LoginPasswordChanged(s) => {
                self.user_info.password = s;
                Command::none()
            }
            SimPlexMessage::DoLogIn => Command::perform(
                PlexUser::authenticate(
                    self.user_info.username.clone(),
                    self.user_info.password.to_owned(),
                ),
                SimPlexMessage::LoginResult,
            ),
            SimPlexMessage::LoginResult(login_opt) => {
                if let Some(user) = login_opt {
                    let token = user.auth_token.clone();

                    self.user_info.auth_token = Some(user.auth_token.clone());
                    self.state = AppState::MainView(user);
                    Command::perform(PlexServer::query_servers(token), |_| SimPlexMessage::Pass)
                } else {
                    println!("Error logging in.");
                    Command::none()
                }
            }
        }
    }

    fn view(self: &SimPlexApp) -> Element<Self::Message> {
        match &self.state {
            AppState::Login => self._view_login(),
            AppState::MainView(_) => self._view_main(),
        }
    }
}

impl SimPlexApp {
    fn _view_main(&self) -> Element<SimPlexMessage> {
        let view = column()
            .align_items(Alignment::Center)
            .push(text("Logged in"));

        container(view).into()
    }

    fn _view_login(&self) -> Element<SimPlexMessage> {
        // TODO: save username (not password) to disk, along with auth token if we got one
        // TODO: if there's an auth token, try to get some info
        // TODO: as soon as you get some good info, switch to the MainView state
        let view = column()
            .push(
                row()
                    .padding(20)
                    .spacing(20)
                    .push(
                        column()
                            .push(text("Username/Email").size(24))
                            .push(text("Password").size(24)),
                    )
                    .push(
                        column()
                            .push(
                                text_input(
                                    "username/email",
                                    &self.user_info.username,
                                    SimPlexMessage::LoginUsernameChanged,
                                )
                                .width(Length::Units(250))
                                .padding(4)
                                .size(24),
                            )
                            .push(
                                text_input(
                                    "password",
                                    &self.user_info.password,
                                    SimPlexMessage::LoginPasswordChanged,
                                )
                                .size(24)
                                .width(Length::Units(250))
                                .padding(4)
                                .password(),
                            ),
                    ),
            )
            .align_items(Alignment::Center)
            .push(button("Log in").on_press(SimPlexMessage::DoLogIn));

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
    let login_info: LoginInfo = from_str(&login_file_data)?;

    SimPlexApp::run(Settings::with_flags(login_info))?;

    // PlexServer::from_xml("".into());

    Ok(())
}
