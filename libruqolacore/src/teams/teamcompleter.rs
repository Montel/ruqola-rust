/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TeamCompleter {
    pub name: String,
    pub fname: String,
    #[serde(rename = "_id")]
    pub identifier: String,
}

impl Default for TeamCompleter {
    fn default() -> Self {
        TeamCompleter::new()
    }
}

impl TeamCompleter {
    pub fn new() -> Self {
        TeamCompleter {
            name: String::default(),
            fname: String::default(),
            identifier: String::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teams::teamcompleter::TeamCompleter;

    #[test]
    fn test_is_invalid_by_default() {
        let b = TeamCompleter::new();
        assert!(b.name.is_empty());
        assert!(b.fname.is_empty());
        assert!(b.identifier.is_empty());
    }
}
