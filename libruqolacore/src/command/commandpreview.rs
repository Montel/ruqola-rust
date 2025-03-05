/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;

#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub enum TypePreview {
    #[default]
    Unknown,
    #[serde(alias = "image")]
    Image,
    #[serde(alias = "video")]
    Video,
    #[serde(alias = "audio")]
    Audio,
    #[serde(alias = "text")]
    Text,
    #[serde(alias = "other")]
    Other,
}

fn typepreview_unknown() -> TypePreview {
    TypePreview::Unknown
}

#[derive(Clone, Default, Deserialize, Debug)]
pub struct CommandPreview {
    pub id: String,
    pub value: String,
    #[serde(rename = "type")]
    #[serde(default = "typepreview_unknown")]
    pub type_preview: TypePreview,
}

impl CommandPreview {
    pub fn new() -> Self {
        CommandPreview {
            id: String::new(),
            value: String::new(),
            type_preview: TypePreview::Unknown,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.type_preview != TypePreview::Unknown && !self.id.is_empty() && !self.value.is_empty()
    }
}

/*
Debug output for CommandPreview
*/
impl fmt::Display for CommandPreview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CommandPreview( id: {}, value: {}, type_preview {:?})",
            self.id, self.value, self.type_preview
        )
    }
}
