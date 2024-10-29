/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessageInfo {
    #[serde(rename = "_id")]
    pub identifier: String,
    #[serde(rename = "msg")]
    pub message: String,
    pub alias: String,
    pub emoji: String,
    pub avatar: String,
    #[serde(default)]
    pub tmid: String,
    #[serde(default)]
    pub drid: String,
    pub role: String,
    #[serde(default)]
    pub tcount: i64,
    #[serde(default)]
    pub dcount: i64,
    // TODO add type
    pub rid: String,
}

impl Default for MessageInfo {
    fn default() -> Self {
        MessageInfo::new()
    }
}

impl MessageInfo {
    pub fn new() -> Self {
        MessageInfo {
            identifier: String::default(),
            message: String::default(),
            alias: String::default(),
            emoji: String::default(),
            avatar: String::default(),
            tmid: String::default(),
            drid: String::default(),
            role: String::default(),
            rid: String::default(),
            tcount: 0,
            dcount: 0,
        }
    }
    pub fn parse(filename: &str) -> MessageInfo {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

/*
Debug output for MessageInfo
*/
impl fmt::Display for MessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(message identifier: {}, message: {}, alias: {})",
            self.identifier, self.message, self.alias
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::messageinfo::MessageInfo;

    #[test]
    fn test_is_empty() {
        let b = MessageInfo::new();
        assert!(b.identifier.is_empty());
        assert!(b.alias.is_empty());
        assert!(b.emoji.is_empty());
        assert!(b.avatar.is_empty());
        assert!(b.tmid.is_empty());
        assert!(b.drid.is_empty());
        assert!(b.role.is_empty());
        assert_eq!(b.dcount, 0);
        assert_eq!(b.tcount, 0);
        assert!(b.rid.is_empty());
    }
}
