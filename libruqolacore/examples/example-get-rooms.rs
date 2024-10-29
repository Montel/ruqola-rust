/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use config_file::FromConfigFile;
use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
use librocketchatrestapi::methods::APIMethod;
use librocketchatrestapi::methods::GetRoomsMethod;
mod helper;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Modify path
    let config =
        helper::helper::Config::from_config_file(helper::helper::default_config_path()).unwrap();
    let loginsettings = AuthenticationType::Login(LoginSettings {
        username: config.username,
        password: config.password,
    });

    let result = GetRoomsMethod {
        settings: loginsettings,
        server_url: config.domain,
    }
    .call()
    .await;

    println!("result {:#?}", result);
    //let rooms: Rooms =
    //serde_json::from_str(&result.expect("Malformed")).expect("JSON was not well-formatted");
    // println!("Commands info! {:?}", rooms);

    // println!("result {:?}", result);
    Ok(())
}
