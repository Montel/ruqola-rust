/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TeamInfo {
    pub team_id: String,
    pub rooms_count: i64,
    pub main_team: bool,
    pub auto_join: bool,
}

impl Default for TeamInfo {
    fn default() -> Self {
        TeamInfo::new()
    }
}

impl TeamInfo {
    pub fn new() -> Self {
        TeamInfo {
            team_id: String::default(),
            rooms_count: 0,
            main_team: false,
            auto_join: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teams::teaminfo::TeamInfo;

    #[test]
    fn test_is_invalid_by_default() {
        let b = TeamInfo::new();
        assert!(!b.main_team);
        assert!(!b.auto_join);
        assert_eq!(b.rooms_count, 0);
        assert!(b.team_id.is_empty());
    }
}
