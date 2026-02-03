/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct CustomEmoji {
    pub emoji_identifier: String,
    #[serde(rename = "_id")]
    pub identifier: String,
    pub extension: String,
    pub name: String,
    pub aliases: Vec<String>,
    /*
    String cachedHtml = '';
    int updatedAt = 0;
    */
}

impl Default for CustomEmoji {
    fn default() -> Self {
        CustomEmoji::new()
    }
}

impl CustomEmoji {
    fn new() -> Self {
        CustomEmoji {
            emoji_identifier: String::default(),
            identifier: String::default(),
            extension: String::default(),
            name: String::default(),
            aliases: Vec::default(),
        }
    }
}

/*
Debug output for Invitation
*/
impl fmt::Display for CustomEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomEmoji(emojiIdentifier {})", self.emoji_identifier)
    }
}
