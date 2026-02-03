/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TeamRoom {
    pub name: String,
    pub fname: String,
    #[serde(rename = "_id")]
    pub identifier: String,
    #[serde(rename = "teamDefault")]
    pub auto_join: bool,
}

impl Default for TeamRoom {
    fn default() -> Self {
        TeamRoom::new()
    }
}

impl TeamRoom {
    pub fn new() -> Self {
        TeamRoom {
            name: String::default(),
            fname: String::default(),
            identifier: String::default(),
            auto_join: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teams::teamroom::TeamRoom;

    #[test]
    fn test_is_invalid_by_default() {
        let b = TeamRoom::new();
        assert!(b.name.is_empty());
        assert!(b.fname.is_empty());
        assert!(b.identifier.is_empty());
        assert!(!b.auto_join);
    }
}
