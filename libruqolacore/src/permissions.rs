/*
 * SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::permission::Permission;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Permissions {
    pub remove: Vec<Permission>,
    pub update: Vec<Permission>,
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::new()
    }
}

impl Permissions {
    pub fn new() -> Self {
        Permissions {
            remove: Vec::<Permission>::default(),
            update: Vec::<Permission>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<Permissions>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::permissions::Permissions;

    // For test !
    pub fn parse(filename: &str) -> Permissions {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_empty() {
        let b: Permissions = Permissions::new();
        assert!(b.update.is_empty());
        assert!(b.remove.is_empty());
    }
    #[test]
    fn test_parsing() {
        {
            // Load file
            let permission = parse("src/data/permissions/permissions.json");
            assert_eq!(permission.update.len(), 1147);
            assert_eq!(permission.remove.len(), 0);
        }
    }
}
