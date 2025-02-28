/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
pub struct CommandPreview {
    pub id: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_preview: String,
}

impl CommandPreview {
    pub fn new() -> Self {
        CommandPreview {
            id: String::new(),
            value: String::new(),
            type_preview: String::new(),
        }
    }
}

/*
Debug output for CommandPreview
*/
impl fmt::Display for CommandPreview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CommandPreview( id: {}, value: {}, type_preview {})",
            self.id, self.value, self.type_preview
        )
    }
}
