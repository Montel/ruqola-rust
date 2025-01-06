/*
 * SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use config_file::FromConfigFile;
use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
use librocketchatrestapi::methods::APIMethod;
use librocketchatrestapi::methods::PermissionsListAllMethod;
use libruqolacore::permissions::Permissions;
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

    let result = PermissionsListAllMethod {
        settings: loginsettings,
        server_url: config.domain,
    }
    .call()
    .await;
    // println!("result {:?}", result);
    let mut permissions = Permissions::new();
    permissions.parse_elements(&result.expect("Malformed"));
    println!("Permissions info! {:#?}", permissions);
    Ok(())
}
