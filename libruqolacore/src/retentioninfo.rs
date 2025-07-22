/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct RetentionInfo {
    #[serde(rename = "maxAge")]
    pub max_age: i64,
    pub enabled: bool,
    #[serde(rename = "overrideGlobal")]
    pub override_global: bool,
    #[serde(rename = "excludePinned")]
    pub exclude_pinned: bool,
    #[serde(rename = "filesOnly")]
    pub files_only: bool,
}

impl Default for RetentionInfo {
    fn default() -> Self {
        RetentionInfo::new()
    }
}

impl RetentionInfo {
    pub fn new() -> Self {
        RetentionInfo {
            max_age: -1,
            enabled: false,
            override_global: false,
            exclude_pinned: false,
            files_only: false,
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<RetentionInfo>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::retentioninfo::RetentionInfo;

    // For test !
    pub fn parse(filename: &str) -> RetentionInfo {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_empty() {
        let b: RetentionInfo = RetentionInfo::new();
        assert!(!b.enabled);
        assert!(!b.override_global);
        assert!(!b.exclude_pinned);
        assert!(!b.files_only);
        assert_eq!(b.max_age, -1);
    }
    #[test]
    fn test_parsing() {
        {
            // Load file
            //let permission = parse("src/data/RetentionInfo/RetentionInfo.json");
        }
    }
}
