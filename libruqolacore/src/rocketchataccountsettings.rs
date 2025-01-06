/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

extern crate ini;
use ini::Ini;
use std::fmt;
#[derive(Clone, Debug)]
pub struct RocketChatAccountSettings {
    pub account_name: String,
    pub display_name: String,
    pub server_url_name: String,
    pub user_name: String,
    pub password: String,
    pub enabled: bool,
}

impl Default for RocketChatAccountSettings {
    fn default() -> Self {
        RocketChatAccountSettings::new()
    }
}

impl RocketChatAccountSettings {
    pub fn new() -> Self {
        RocketChatAccountSettings {
            account_name: String::default(),
            display_name: String::default(),
            server_url_name: String::default(),
            user_name: String::default(),
            password: String::default(),
            enabled: true,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.user_name.is_empty()
            && !self.account_name.is_empty()
            && !self.server_url_name.is_empty()
    }
    // Load config file
    pub fn load_settings(&mut self, file_name: String) {
        match Ini::load_from_file(file_name) {
            Ok(conf) => {
                let general = conf.section(Some("General")).unwrap();
                if let Some(account_name) = general.get("accountName") {
                    self.account_name = account_name.to_string();
                }
                if let Some(user_name) = general.get("username") {
                    self.user_name = user_name.to_string();
                }
                if let Some(display_name) = general.get("displayName") {
                    self.display_name = display_name.to_string();
                }
                if let Some(server_url_name) = general.get("serverURL") {
                    self.server_url_name = server_url_name.to_string();
                }
                // Don't store password here. But at the moment we will do it.
                if let Some(password) = general.get("password") {
                    self.password = password.to_string();
                }
                // TODO enabled
            }
            Err(e) => println!("Error parsing file: invalid file {}", e),
        }
        // https://crates.io/crates/libsecret/0.2.0
        // TODO add password ??? load from service secret gnome ?
    }
    pub fn write_settings(&self, file_name: String) {
        let mut conf = Ini::new();
        conf.with_section(Some("General"))
            .set("accountName", self.account_name.clone())
            .set("username", self.user_name.clone())
            .set("displayName", self.display_name.clone())
            .set("serverURL", self.server_url_name.clone())
            .set("password", self.password.clone());
        conf.write_to_file(file_name).unwrap();
    }
}

/*
Debug output for RocketChatAccountSettings
*/
impl fmt::Display for RocketChatAccountSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(account name: {}, display name: {}, server url: {}, user_name: {}, password: {}, enabled: {})",
        self.account_name, self.display_name, self.server_url_name, self.user_name, self.password, self.enabled)
    }
}
