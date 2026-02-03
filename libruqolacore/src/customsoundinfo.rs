/*
 * SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(default)]
pub struct CustomSoundInfo {
    pub name: String,
    #[serde(rename = "_id")]
    pub identifier: String,
    pub extension: String,
}

impl CustomSoundInfo {
    pub fn new() -> Self {
        CustomSoundInfo {
            name: String::default(),
            identifier: String::default(),
            extension: String::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<CustomSoundInfo>(json) {
            *self = val;
        }
    }
}

/*
Debug output for CustomSoundInfo
*/
impl fmt::Display for CustomSoundInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "customInfo identifier: {}, custominfo extension: {})",
            self.identifier, self.extension
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::customsoundinfo::CustomSoundInfo;
    use std::fs::File;

    pub fn parse(filename: &str) -> CustomSoundInfo {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_custom_sound_info_default() {
        let custom_sound_info = CustomSoundInfo::new();
        assert!(custom_sound_info.name.is_empty());
        assert!(custom_sound_info.identifier.is_empty());
        assert!(custom_sound_info.extension.is_empty());
    }
}
