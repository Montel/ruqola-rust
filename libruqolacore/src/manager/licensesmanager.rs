/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct LicensesManager {
    pub license: LicensesInfo,
}

impl Default for LicensesManager {
    fn default() -> Self {
        LicensesManager::new()
    }
}

impl LicensesManager {
    pub fn new() -> Self {
        LicensesManager {
            license: LicensesInfo::default(),
        }
    }
    pub fn has_license(&self, name: String) -> bool {
        self.license.active_modules.contains(&name)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct LicensesInfo {
    #[serde(rename = "activeModules")]
    pub active_modules: Vec<String>,
}
impl LicensesInfo {
    fn new() -> Self {
        LicensesInfo {
            active_modules: Vec::<String>::default(),
        }
    }
}

impl Default for LicensesInfo {
    fn default() -> Self {
        LicensesInfo::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::licensesmanager::LicensesManager;
    use std::fs::File;
    pub fn parse(filename: &str) -> LicensesManager {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_default_values() {
        let r = LicensesManager::new();
        assert!(r.license.active_modules.is_empty());
    }

    #[test]
    fn test_contains_license() {
        let mut r = LicensesManager::new();
        r.license.active_modules.push("dd".to_string());
        r.license.active_modules.push("license2".to_string());

        assert!(!r.license.active_modules.is_empty());
        assert!(r.license.active_modules.len() == 2);
        assert!(r.has_license(String::from("dd")));
        assert!(!r.has_license(String::from("dd2")));
    }

    #[test]
    fn test_parse_data() {
        {
            // Load file
            let result = parse("src/data/licensesmanager/test1.json");
            let result_list = vec![
                String::from("auditing"),
                String::from("canned-responses"),
                String::from("ldap-enterprise"),
                String::from("livechat-enterprise"),
                String::from("voip-enterprise"),
                String::from("omnichannel-mobile-enterprise"),
                String::from("engagement-dashboard"),
                String::from("push-privacy"),
                String::from("scalability"),
                String::from("teams-mention"),
                String::from("saml-enterprise"),
                String::from("oauth-enterprise"),
                String::from("device-management"),
                String::from("federation"),
                String::from("videoconference-enterprise"),
                String::from("message-read-receipt"),
                String::from("outlook-calendar"),
                String::from("hide-watermark"),
                String::from("custom-roles"),
                String::from("accessibility-certification"),
            ];
            assert_eq!(result.license.active_modules, result_list);
        }
    }
}
