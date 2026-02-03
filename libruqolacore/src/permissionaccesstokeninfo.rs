/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct PermissionAccessTokenInfo {
    #[serde(rename = "lastTokenPart")]
    pub last_token_part: String,
    pub name: String,
    #[serde(rename = "bypassTwoFactor")]
    pub bypass_two_factor: bool,
    #[serde(rename = "createdAt")]
    pub create_date_time: DateTime<Utc>,
}

impl Default for PermissionAccessTokenInfo {
    fn default() -> Self {
        PermissionAccessTokenInfo::new()
    }
}

impl PermissionAccessTokenInfo {
    pub fn new() -> Self {
        PermissionAccessTokenInfo {
            last_token_part: String::default(),
            name: String::default(),
            bypass_two_factor: false,
            create_date_time: DateTime::default(),
        }
    }
}

/*
Debug output for Permission
*/
impl fmt::Display for PermissionAccessTokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(last_token_part{}, name: {:?}, bypass_two_factor: {})",
            self.last_token_part, self.name, self.bypass_two_factor
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::permissionaccesstokeninfo::PermissionAccessTokenInfo;

    #[test]
    fn test_default_value() {
        let permission_access_token = PermissionAccessTokenInfo::new();
        assert!(permission_access_token.last_token_part.is_empty());
        assert!(permission_access_token.name.is_empty());
        assert!(!permission_access_token.bypass_two_factor);
    }
}
