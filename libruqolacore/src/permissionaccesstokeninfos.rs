/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::permissionaccesstokeninfo::PermissionAccessTokenInfo;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct PermissionAccessTokenInfos {
    #[serde(rename = "tokens")]
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

    use crate::permissionaccesstokeninfo::PermissionAccessTokenInfo;
    use crate::permissionaccesstokeninfos::PermissionAccessTokenInfos;
    use chrono::{DateTime, Utc};
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

    #[test]
    fn test_load_empty() {
        let b = parse("src/data/personalaccesstokeninfos/empty-token.json");
        assert!(b.permission_access_token_infos.is_empty());
    }

    #[test]
    fn test_load_four_tokens() {
        let b = parse("src/data/personalaccesstokeninfos/four-tokens.json");
        assert!(!b.permission_access_token_infos.is_empty());
        let mut lst: Vec<PermissionAccessTokenInfo> = Vec::new();
        {
            let mut i = PermissionAccessTokenInfo::new();
            i.bypass_two_factor = false;
            i.last_token_part = String::from("I6mHG3");
            i.name = String::from("test1");

            i.create_date_time = DateTime::from_timestamp_millis(1741856600248)
                .unwrap()
                .with_timezone(&Utc);

            lst.push(i);
        }
        {
            let mut i = PermissionAccessTokenInfo::new();
            i.bypass_two_factor = false;
            i.last_token_part = String::from("ClGA8G");
            i.name = String::from("test2");

            i.create_date_time = DateTime::from_timestamp_millis(1741856613045)
                .unwrap()
                .with_timezone(&Utc);

            lst.push(i);
        }
        {
            let mut i = PermissionAccessTokenInfo::new();
            i.bypass_two_factor = false;
            i.last_token_part = String::from("eM7RyS");
            i.name = String::from("test3");
            i.create_date_time = DateTime::from_timestamp_millis(1741856625084)
                .unwrap()
                .with_timezone(&Utc);

            lst.push(i);
        }
        {
            let mut i = PermissionAccessTokenInfo::new();
            i.bypass_two_factor = true;
            i.last_token_part = String::from("nGVprC");
            i.name = String::from("test4");
            i.create_date_time = DateTime::from_timestamp_millis(1741856839672)
                .unwrap()
                .with_timezone(&Utc);
            lst.push(i);
        }
        assert_eq!(b.permission_access_token_infos, lst);
    }
}
