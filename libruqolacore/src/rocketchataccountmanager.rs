/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

extern crate xdg;

use crate::rocketchataccount::RocketChatAccount;
use crate::rocketchataccountsettings::RocketChatAccountSettings;

use std::fs;
use std::path::Path;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub enum CommandFromGui {
    ConnectAccount {
        account_name: String,
    },
    DisconnectAccount {
        account_name: String,
    },
    ChangeAccount {
        account_name: String,
        account_settings: RocketChatAccountSettings,
    },
    AddAccount {
        account_settings: RocketChatAccountSettings,
    },
    RemoveAccount {
        index_name: i32,
    },
    SendMessage {
        message: String,
        room_id: String,
    },
    SwitchRoom {
        room_id: String,
    },
}

pub enum CommandToBackend {
    SendMessage { message: String, room_id: String },
    UpdateAccountList,
}

pub struct RocketChatAccountManager {
    // Vector of rocketchataccount settings
    pub rocketchat_accounts: Vec<RocketChatAccount>,
    pub receiver: Option<mpsc::Receiver<CommandFromGui>>,
    // TODO currentAccount
}

impl Default for RocketChatAccountManager {
    fn default() -> Self {
        RocketChatAccountManager::new()
    }
}

impl RocketChatAccountManager {
    pub fn new() -> Self {
        Self {
            rocketchat_accounts: Vec::<RocketChatAccount>::new(),
            receiver: None,
        }
    }

    pub fn set_io_event_rx(&mut self, rx: mpsc::Receiver<CommandFromGui>) {
        self.receiver = Some(rx);
    }

    pub fn send_message(&self, message: String, room_id: String) {
        // TODO send message to current account manager
        println!("SEND MESSAGE **********************************");
    }

    pub async fn initialize_accounts(&mut self) {
        <Vec<RocketChatAccount> as Clone>::clone(&self.rocketchat_accounts)
            .into_iter()
            .for_each(|account: RocketChatAccount| {
                tokio::spawn({
                    let (sender, mut receiver) = mpsc::unbounded_channel::<CommandToBackend>();
                    account.build(receiver, sender)
                });
            });
    }

    pub fn load_accounts(&mut self) {
        // https://www.youtube.com/watch?v=4EmKgrzHfv4
        let xdg_dirs = xdg::BaseDirectories::with_prefix("ruqola-slint").unwrap();
        // Search all files in directory
        let current_file = xdg_dirs.get_config_home();
        println!("chemin {:?}", current_file);
        let path = Path::new(&current_file);
        if !path.exists() {
            fs::create_dir(path).expect("Impossible to create {path}");
        }

        let directories = fs::read_dir(path)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().unwrap().is_dir())
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        println!("Directories: {:?}", directories);

        for entry in directories {
            let path = entry.as_path();
            let ruqola_config_filename = Path::new("ruqola.conf");
            let ruqola_conf_path = path.join(ruqola_config_filename);
            if ruqola_conf_path.exists() {
                self.load_account(ruqola_conf_path.to_str().unwrap().to_string());
                println!("File path: {:?}", entry.as_path());
            }
        }
    }

    // TODO look at how we can use it only in test => remove pub when test
    pub fn load_account(&mut self, file_name: String) {
        println!("File path**** : {:?}", file_name);
        let mut rocketaccount = RocketChatAccount::new();
        rocketaccount.load_settings(file_name);
        rocketaccount
            .account_backend
            .ddpclient_builder
            .set_websocket_url(rocketaccount.account_settings.server_url_name.clone());
        let ddpsettings: libauthenticationbase::authenticationsettings::AuthenticationType =
            libauthenticationbase::authenticationsettings::AuthenticationType::Login(
                libauthenticationbase::authenticationsettings::LoginSettings {
                    username: rocketaccount.account_settings.user_name.clone(),
                    password: rocketaccount.account_settings.password.clone(),
                },
            );
        // Initialize ddpsetting
        rocketaccount
            .account_backend
            .ddpclient_builder
            .set_settings(ddpsettings);

        let r2 = rocketaccount.clone();
        if r2.account_settings.is_valid() {
            self.rocketchat_accounts.push(rocketaccount);
        }
    }
    // Return list of accounts names
    pub fn list_accounts(&self) -> Vec<String> {
        let mut account_list_names = Vec::<String>::new();
        for account in &self.rocketchat_accounts {
            if account.account_settings.display_name.is_empty() {
                account_list_names.push(account.account_settings.account_name.clone());
            } else {
                account_list_names.push(account.account_settings.display_name.clone());
            }
        }
        account_list_names
    }
    // Remove account name
    pub fn remove_account(&mut self, index: usize) {
        if index < self.rocketchat_accounts.capacity() {
            println!("Before {:?}", self.rocketchat_accounts.capacity());
            let name = self.rocketchat_accounts[index]
                .account_settings
                .clone()
                .display_name;
            self.rocketchat_accounts
                .retain(|x| x.account_settings.display_name == name);
            println!("After {:?}", self.rocketchat_accounts.capacity());
            // TODO remove on filesystem too
        }
    }
    // Add account
    pub fn add_account(&mut self, account_settings: RocketChatAccountSettings) {
        println!("Add account !");
        if account_settings.is_valid() {
            println!("Add account  is Valid !");
            let mut rocketaccount = RocketChatAccount::new();
            rocketaccount.account_settings = account_settings;
            self.rocketchat_accounts.push(rocketaccount);
        }
    }

    // Modify account
    pub fn modify_account(
        &mut self,
        account_settings: RocketChatAccountSettings,
        account_name: String,
    ) {
        println!("modify account !");
        // TODO search account
        if account_settings.is_valid() {
            println!("Add account  is Valid !");
            let mut rocketaccount = RocketChatAccount::new();
            rocketaccount.account_settings = account_settings;
            self.rocketchat_accounts.push(rocketaccount);
        }
    }

    pub fn write_account(&self) {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("ruqola-slint").unwrap();
        let current_file = xdg_dirs.get_config_home();
        for x in &self.rocketchat_accounts {
            let path = Path::new(&current_file).join(x.account_settings.account_name.clone());

            let fillpath = path.clone();
            if !path.exists() {
                fs::create_dir(path).expect("Impossible to create {path}");
            }
            let path = fillpath.join("ruqola.conf");
            x.account_settings
                .write_settings(String::from(path.to_str().unwrap()));
        }
    }

    // Search element!
    pub fn account_settings(&self, account_name: String) -> RocketChatAccountSettings {
        let mut account = RocketChatAccountSettings::new();
        println!("search element {:?}", account_name);
        match self
            .rocketchat_accounts
            .iter()
            .find(|&x| x.account_settings.display_name == account_name)
        {
            Some(x) => {
                println!("Element {} found", x.account_settings.display_name);
                account = x.account_settings.clone();
            }
            None => println!("Element not found"),
        }
        account
    }

    pub fn account_settings_from_index(&self, index: usize) -> Option<RocketChatAccountSettings> {
        if index < self.rocketchat_accounts.capacity() {
            Some(self.rocketchat_accounts[index].account_settings.clone())
        } else {
            None
        }
    }
}

lazy_static! {
    pub static ref ACCOUNT_MANAGER_STATIC: Mutex<RocketChatAccountManager> =
        Mutex::new(RocketChatAccountManager::new());
}
