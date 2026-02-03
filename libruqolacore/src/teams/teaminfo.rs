use std::fmt;

/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct TeamInfo {
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "roomsCount")]
    pub rooms_count: i64,
    #[serde(rename = "teamMain")]
    pub main_team: bool,
    #[serde(rename = "teamDefault")]
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
    pub fn has_team_room(&self) -> bool {
        !self.main_team && !self.team_id.is_empty()
    }
}

/*
Debug output for TeamInfo
*/
impl fmt::Display for TeamInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TeamInfo(teamId: {}, rooms_count: {}, main_team: {}, auto_join: {})",
            self.team_id, self.rooms_count, self.main_team, self.auto_join
        )
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
        assert!(!b.has_team_room());
    }

    #[test]
    fn test_verify_has_team_room() {
        let mut b = TeamInfo::new();
        assert!(!b.has_team_room());
        b.main_team = true;
        b.team_id = String::from("ff");
        assert!(!b.has_team_room());
        b.main_team = false;
        assert!(b.has_team_room());

        b.team_id.clear();
        assert!(!b.has_team_room());
    }
}
