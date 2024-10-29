/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct AppsLanguageInfo {
    translated_map: HashMap<String, String>,
}

impl Default for AppsLanguageInfo {
    fn default() -> Self {
        AppsLanguageInfo::new()
    }
}

impl AppsLanguageInfo {
    pub fn new() -> Self {
        AppsLanguageInfo {
            ..Default::default()
        }
    }
    //pub fn parse(filename: &str) -> AppsLanguageInfo {
    // serde_json::from_str(filename).expect("JSON was not well-formatted")
    //}
}
