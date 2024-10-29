/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Permission {
    #[serde(rename = "_id")]
    pub identifier: String,
    pub roles: Vec<String>,
}

impl Default for Permission {
    fn default() -> Self {
        Permission::new()
    }
}

impl Permission {
    pub fn new() -> Self {
        Permission {
            identifier: String::default(),
            roles: vec![],
        }
    }
}

/*
Debug output for Permission
*/
impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(identifier{}, roles: {:?})",
            self.identifier, self.roles
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::permission::Permission;
    use std::fs::File;

    // Using by test
    pub fn parse(filename: &str) -> Permission {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_default_value() {
        let invit = Permission::new();
        assert!(invit.identifier.is_empty());
        assert!(invit.roles.is_empty());
    }

    #[test]
    fn test_parsing() {
        {
            // Load file
            let permission = parse("src/data/permission/permission1.json");
            assert_eq!(permission.identifier, "pin-message");
            assert_eq!(
                permission.roles,
                ["owner", "moderator", "admin", "global-moderator"]
            );
        }
        {
            // Load file
            let permission = parse("src/data/permission/permission2.json");
            assert_eq!(permission.identifier, "create-c");
            assert_eq!(permission.roles, ["admin", "user", "bot", "app"]);
        }
    }
}
