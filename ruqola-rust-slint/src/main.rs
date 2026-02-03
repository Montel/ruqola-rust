/*
 * SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

// TODO create a module for it!
mod commandlineoptions;
mod ruqolawindow;
mod utils;

use crate::commandlineoptions::Cli;
use clap::Parser;
use libruqolacore::rocketchataccountmanager::CommandFromGui;
use libruqolacore::rocketchataccountmanager::CommandToBackend;
use libruqolacore::rocketchataccountmanager::RocketChatAccountManager;
use ruqolawindow::Gui;

use futures::executor::block_on;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/lang/"));
    println!("Hello, world!");
    let args = Cli::parse();
    match args.command {
        Some(commandlineoptions::Commands::Account { accountname }) => {
            // TODO connect to specific account name!
            println!("Account name {}", accountname);
        }
        Some(commandlineoptions::Commands::ListAccount {}) => {
            let mut manager = RocketChatAccountManager::new();
            manager.load_accounts();
            let accounts = manager.list_accounts();
            for acc in &accounts {
                println!("Account name {acc}");
            }
        }
        Some(commandlineoptions::Commands::Console {}) => {
            // Hostname
            let mut host_name = String::new();
            println!("Enter the host name :");
            std::io::stdin().read_line(&mut host_name).unwrap();
            println!("Host name is : {}", host_name);

            // User Name
            let mut user_name = String::new();
            println!("Enter the username :");
            std::io::stdin().read_line(&mut user_name).unwrap();
            println!("username is : {}", user_name);

            // Password
            let mut password = String::new();
            println!("Enter the password :");
            std::io::stdin().read_line(&mut password).unwrap();
            println!("Password is : {}", password);

            // TODO use libddpclient

            block_on(utils::connect_to_server(host_name, user_name, password));
        }
        None => {
            log::info!(target: "commandline", "Show interface");
            println!("show Interface");

            // create command channel
            let (transmeter_data_to_ui, mut receiver_data_from_ui) =
                mpsc::unbounded_channel::<CommandFromGui>();

            // create event channel
            let (transmeter_event_data_to_backend, mut receiver_event_data_from_backend) =
                mpsc::unbounded_channel::<CommandToBackend>();

            let mut manager = RocketChatAccountManager::new();
            manager.load_accounts();
            manager.initialize_accounts().await;
            tokio::spawn(async move {
                while let Some(message) = receiver_data_from_ui.recv().await {
                    match message {
                        CommandFromGui::ChangeAccount {
                            account_name,
                            account_settings,
                        } => {
                            println!("change account {account_name}, {account_settings}");
                            manager.modify_account(account_settings, account_name);
                            manager.write_account();
                            transmeter_event_data_to_backend
                                .send(CommandToBackend::UpdateAccountList)
                                .unwrap();
                        }
                        CommandFromGui::AddAccount { account_settings } => {
                            println!("Add new account {:?}", account_settings);
                            manager.add_account(account_settings);
                            manager.write_account();
                            transmeter_event_data_to_backend
                                .send(CommandToBackend::UpdateAccountList)
                                .unwrap();
                        }
                        CommandFromGui::RemoveAccount { index_name } => {
                            println!("Remove account {:?}", index_name);
                            manager.remove_account(index_name as usize);
                            transmeter_event_data_to_backend
                                .send(CommandToBackend::UpdateAccountList)
                                .unwrap();
                        }
                        CommandFromGui::SendMessage { message, room_id } => {
                            println!("SEND MESSAGE  {:?}", message);
                            manager.send_message(message, room_id);
                        }
                        CommandFromGui::ConnectAccount { account_name } => {
                            println!("CONNECT ACCOUNT {:?}", account_name);
                        }
                        CommandFromGui::DisconnectAccount { account_name } => {
                            println!("DISCONNECT ACCOUNT{:?}", account_name);
                        }
                        CommandFromGui::SwitchRoom { room_id } => {
                            println!("Switch room {:?}", room_id);
                        }
                    }
                }
            });

            tokio::spawn(async move {
                while let Some(message) = receiver_event_data_from_backend.recv().await {
                    match message {
                        CommandToBackend::SendMessage { room_id, message } => {
                            println!(
                                "send mssage*************{:?} message {:?}",
                                room_id, message
                            );
                        }
                        CommandToBackend::UpdateAccountList => {
                            println!("Update account list");
                        }
                    }
                }
            });
            Gui::ui_loop(transmeter_data_to_ui).await;
        }
    }
}
