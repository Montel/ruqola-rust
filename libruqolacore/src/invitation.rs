/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Invitation {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "_id")]
    pub identifier: String,
    #[serde(rename = "rid")]
    pub room_identifier: String,
    pub uses: u64,
    #[serde(rename = "maxUses")]
    pub max_uses: u64,
    #[serde(rename = "expires")]
    pub expire_date_time: DateTime<Utc>,

    #[serde(rename = "createdAt")]
    pub create_date_time: DateTime<Utc>,
}

impl Default for Invitation {
    fn default() -> Self {
        Invitation::new()
    }
}

impl Invitation {
    pub fn new() -> Self {
        Invitation {
            user_id: String::default(),
            identifier: String::default(),
            room_identifier: String::default(),
            uses: 0,
            max_uses: 0,
            expire_date_time: DateTime::default(),
            create_date_time: DateTime::default(),
        }
    }
}

/*
Debug output for Invitation
*/
impl fmt::Display for Invitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(user identifier {}, identifier{}, room_id{}, uses {}, max_uses{})",
            self.user_id, self.identifier, self.room_identifier, self.uses, self.max_uses
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::invitation::Invitation;
    use std::fs::File;

    // Using by test
    pub fn parse(filename: &str) -> Invitation {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_default_value() {
        let invit = Invitation::new();
        assert_eq!(invit.max_uses, 0);
        assert_eq!(invit.uses, 0);
        assert!(invit.identifier.is_empty());
        assert!(invit.user_id.is_empty());
        assert!(invit.room_identifier.is_empty());
        // assert!(invit.create_date_time.)
    }

    #[test]
    fn test_parsing() {
        // Load file
        let invit = parse("src/data/invitation/inviteinfo.json");
        // Parse invitation
        assert_eq!(invit.uses, 0);
        assert_eq!(invit.max_uses, 25);
        assert_eq!(invit.room_identifier, "n2GWePY4zjG48g7qA");
        assert_eq!(invit.user_id, "H7Q9djXQ4iShzD9T2");
        assert_eq!(invit.identifier, "D2F6of");
        assert_eq!(
            "2021-04-08 06:49:04.571 UTC",
            invit.expire_date_time.to_string()
        );
        assert_eq!(
            "2021-04-07 06:49:04.571 UTC",
            invit.create_date_time.to_string()
        );
    }
}
