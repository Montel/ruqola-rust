/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use std::fmt;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessagePinned {
    #[serde(rename = "pinnedBy")]
    pub pinned_by: String,
    pub pinned: bool,
}

impl Default for MessagePinned {
    fn default() -> Self {
        MessagePinned::new()
    }
}

impl MessagePinned {
    pub fn new() -> Self {
        MessagePinned {
            pinned_by: String::default(),
            pinned: false,
        }
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for MessagePinned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessagePinned(pinned_by: {}, pinned: {})",
            self.pinned_by, self.pinned
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::messagepinned::MessagePinned;

    #[test]
    fn test_is_empty() {
        let b = MessagePinned::new();
        assert!(b.pinned_by.is_empty());
        assert!(!b.pinned);
    }
}
