/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use std::fmt;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessageAttachmentField {
    pub value: String,
    pub title: String,
}

impl Default for MessageAttachmentField {
    fn default() -> Self {
        MessageAttachmentField::new()
    }
}

impl MessageAttachmentField {
    pub fn new() -> Self {
        MessageAttachmentField {
            title: String::default(),
            value: String::default(),
        }
    }
    pub fn parse(filename: &str) -> MessageAttachmentField {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for MessageAttachmentField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(AttachmentField title: {}, value: {})",
            self.title, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::messageattachmentfield::MessageAttachmentField;

    #[test]
    fn test_is_empty() {
        let b = MessageAttachmentField::new();
        assert!(b.value.is_empty());
        assert!(b.title.is_empty());
    }
}
