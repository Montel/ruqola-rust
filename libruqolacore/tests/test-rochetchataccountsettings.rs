/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[cfg(test)]
extern crate libruqolacore;

use libruqolacore::rocketchataccountsettings;

#[test]
fn test_rocketchataccountsettings() {
    let settings = rocketchataccountsettings::RocketChatAccountSettings::new();
    assert!(settings.account_name.is_empty());
    assert!(settings.display_name.is_empty());
    assert!(settings.server_url_name.is_empty());
    assert!(settings.user_name.is_empty());
    assert!(settings.password.is_empty());
    assert!(settings.enabled);
    assert!(!settings.is_valid());
}
