/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub struct MessageTranslations {
    #[serde(rename = "translations")]
    pub translated_string: HashMap<String, String>,
}

impl MessageTranslations {
    pub fn new() -> Self {
        MessageTranslations {
            translated_string: HashMap::new(),
        }
    }
}

/*
Debug output for MessageTranslations
*/
impl fmt::Display for MessageTranslations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageTranslations(translated_string: {:?})",
            self.translated_string
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::messagetranslations::MessageTranslations;

    #[test]
    fn test_is_empty() {
        let b = MessageTranslations::new();
        assert!(b.translated_string.is_empty());
    }
}
