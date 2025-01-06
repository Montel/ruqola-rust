/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
use librocketchatrestapi::methods::APIMethod;
use librocketchatrestapi::methods::GetRoomsMethod;

pub async fn connect_to_server(host_name: String, _user_name: String, password: String) {
    //let password_ssha2 = utils::convert_password_ssha256(password);
    // With REST API we need to use direct password.
    // For websocket we need to convert to ssha256
    let _password_2 = password;
    let trim_host_name = host_name.trim();
    //let server_url = utils::adapt_websocket_url(trim_host_name.to_string());
    let _server_url = trim_host_name.to_string();
    println!(" prepare to get rooms");

    let loginsettings = AuthenticationType::Login(LoginSettings {
        username: "username".to_string(), /*user_name*/
        password: "password_2".to_string(),
    });

    let result = GetRoomsMethod {
        settings: loginsettings,
        server_url: "https://kdab.chat.kdab.com".to_string(), /*server_url*/
    }
    .call()
    .await;
    println!(" GetRoomsMethod result {:?}", result);
}
