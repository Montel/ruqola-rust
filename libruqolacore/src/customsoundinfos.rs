/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::customsoundinfo::CustomSoundInfo;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct CustomSoundInfos {
    pub custom_sounds: Vec<CustomSoundInfo>,
}

impl Default for CustomSoundInfos {
    fn default() -> Self {
        CustomSoundInfos::new()
    }
}

impl CustomSoundInfos {
    pub fn new() -> Self {
        CustomSoundInfos {
            custom_sounds: Vec::<CustomSoundInfo>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<CustomSoundInfos>(json) {
            *self = val
        }
    }
}
