/*
 * SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use config_file::FromConfigFile;
use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
use librocketchatrestapi::methods::APIMethod;
use librocketchatrestapi::methods::GetListCommandsMethod;
use libruqolacore::command::commands::Commands;
mod helper;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Modify path
    let config = helper::helper::Config::from_config_file(helper::helper::default_config_path())
        .expect("file no found");
    let loginsettings = AuthenticationType::Login(LoginSettings {
        username: config.username,
        password: config.password,
    });

    let result = GetListCommandsMethod {
        settings: loginsettings,
        server_url: config.domain,
    }
    .call()
    .await;
    // println!("result {:?}", result);
    let mut commands = Commands::new();
    commands.parse_elements(&result.expect("Malformed"));
    println!("Commands info! {:#?}", commands);
    Ok(())
}
