/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct RoomInfo {
    //TeamInfo mTeamInfo;
    pub usernames: Option<Vec<String>>,
    #[serde(rename = "usersCount")]
    pub users_count: i32,
    pub topic: Option<String>,
    #[serde(rename = "_id")]
    pub identifier: String,
    pub name: Option<String>,
    pub fname: Option<String>,
    pub description: Option<String>,
}

impl Default for RoomInfo {
    fn default() -> Self {
        RoomInfo::new()
    }
}

impl RoomInfo {
    pub fn new() -> Self {
        RoomInfo {
            users_count: 0,
            usernames: None::<Vec<String>>,
            topic: None::<String>,
            identifier: String::default(),
            name: None::<String>,
            fname: None::<String>,
            description: None::<String>,
        }
    }
    pub fn parse(filename: &str) -> RoomInfo {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

#[cfg(test)]
mod tests {
    use crate::room_info::roominfo::RoomInfo;

    #[test]
    fn test_is_invalid_by_default() {
        let b = RoomInfo::new();
        // assert!(!b.is_valid());
    }
}
