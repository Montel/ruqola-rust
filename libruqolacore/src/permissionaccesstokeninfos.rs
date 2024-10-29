/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::permissionaccesstokeninfo::PermissionAccessTokenInfo;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PermissionAccessTokenInfos {
    pub permission_access_token_infos: Vec<PermissionAccessTokenInfo>,
}

impl Default for PermissionAccessTokenInfos {
    fn default() -> Self {
        PermissionAccessTokenInfos::new()
    }
}

impl PermissionAccessTokenInfos {
    pub fn new() -> Self {
        PermissionAccessTokenInfos {
            permission_access_token_infos: Vec::<PermissionAccessTokenInfo>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<PermissionAccessTokenInfos>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::permissionaccesstokeninfos::PermissionAccessTokenInfos;

    // For test !
    pub fn parse(filename: &str) -> PermissionAccessTokenInfos {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_empty() {
        let b = PermissionAccessTokenInfos::new();
        assert!(b.permission_access_token_infos.is_empty());
    }
}
