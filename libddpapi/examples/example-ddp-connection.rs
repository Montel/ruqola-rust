/*
 * SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use libddpapi::ddpclient;

use std::env;
#[tokio::main]
async fn main() {
    let connect_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    // TODO change it
    let user_name = "username".to_string();
    let password = "pwd".to_string();
    let mut ddpclient_builder = ddpclient::DDpClientBuilder::new();
    ddpclient_builder.set_websocket_url(connect_addr);
    let ddpsettings: libauthenticationbase::authenticationsettings::AuthenticationType =
        libauthenticationbase::authenticationsettings::AuthenticationType::Login(
            libauthenticationbase::authenticationsettings::LoginSettings {
                username: user_name.clone(),
                password: password.clone(),
            },
        );
    ddpclient_builder.set_settings(ddpsettings);

    // Verify if we can connect or not
    match ddpclient_builder.build().await {
        Err(b_error) => match b_error {
            ddpclient::BuilderError::MissingUrl => {
                println!("Error url");
            }
            ddpclient::BuilderError::InvalidAccountSettings => {
                println!("Invalid account settings");
            }
        },
        Ok((mut _ddpclient, mut event_receiver)) => {
            println!("CONNECTED ");
            loop {
                tokio::select! {
                    Some(event) = event_receiver.recv() => {
                        match event {
                            ddpclient::Event::MessageReceived(str) => {
                                println!("Message received {:?}", str);
                            },
                            ddpclient::Event::ErrorReceived(str) => {
                                println!("Error received {:?}", str);
                            },
                            ddpclient::Event::ResultReceived(str) => {
                                println!("Result received {:?}", str);
                            },
                            ddpclient::Event::ChangeElementType(element_type) => {
                                println!("Change Elements received {:?}", element_type);
                            },
                            ddpclient::Event::AddElementType(element_type) => {
                                println!("Add Elements received {:?}", element_type);
                                match element_type {
                                    libddpapi::ddpmessage::AddElementType::User(user_added) => {
                                        if user_added.user_name == user_name.clone() {
                                            println!("current user !!!!!");
                                            // _ddpclient.command_sender
                                            // register user
                                        }
                                        println!("USER ADDED")
                                    },
                                    _ => println!("OTHER"),
                                };
                            },
                            ddpclient::Event::RemoveElementType(element_type) => {
                                println!("Remove Elements received {:?}", element_type);
                            },
                        }
                    }
                }
            }
        }
    }
}
