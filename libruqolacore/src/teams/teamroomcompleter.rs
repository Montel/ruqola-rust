/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TeamRoomCompleter {
    pub name: String,
    pub fname: String,
    #[serde(rename = "_id")]
    pub identifier: String,
}

impl Default for TeamRoomCompleter {
    fn default() -> Self {
        TeamRoomCompleter::new()
    }
}

impl TeamRoomCompleter {
    pub fn new() -> Self {
        TeamRoomCompleter {
            name: String::default(),
            fname: String::default(),
            identifier: String::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teams::teamroomcompleter::TeamRoomCompleter;

    #[test]
    fn test_is_invalid_by_default() {
        let b = TeamRoomCompleter::new();
        assert!(b.name.is_empty());
        assert!(b.fname.is_empty());
        assert!(b.identifier.is_empty());
    }
}
