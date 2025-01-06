/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[cfg(test)]
extern crate libruqolacore;

use libruqolacore::rocketchataccountmanager;

#[test]
fn test_rocketchataccountmanager_load_file() {
    let manager = rocketchataccountmanager::RocketChatAccountManager::new();
    // TODO load
    let file_name = String::from("");
    //let account = manager.load_account(file_name);
    // TODO assert_eq!(manager.account_settings.length(), 0);
}
