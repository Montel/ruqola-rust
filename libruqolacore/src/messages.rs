/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::messageinfo::MessageInfo;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Messages {
    pub update: Vec<MessageInfo>, // TODO verify it
}

impl Default for Messages {
    fn default() -> Self {
        Messages::new()
    }
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            update: Vec::<MessageInfo>::default(),
        }
    }
    // For test !
    pub fn parse(filename: &str) -> Messages {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::Messages;

    #[test]
    fn test_is_empty() {
        let b = Messages::new();
        assert!(b.update.is_empty());
    }
}
