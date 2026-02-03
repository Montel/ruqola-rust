/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct ServicePassword {
    pub password: String,
    pub email2_fa_enabled: bool,
    pub totp: bool,
}

impl Default for ServicePassword {
    fn default() -> Self {
        ServicePassword::new()
    }
}

impl ServicePassword {
    pub fn new() -> Self {
        ServicePassword {
            password: String::default(),
            email2_fa_enabled: false,
            totp: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ownuser::servicepassword::ServicePassword;
    use std::fs::File;

    // Using by test
    pub fn parse(filename: &str) -> ServicePassword {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_parsing() {
        {
            // Load file
            // TODO
            //let preferences = parse("src/data/ownuserpreferences/ownuserpreferences1.json");
            //assert_eq!(permission.update.len(), 1147);
            //assert_eq!(permission.remove.len(), 0);
        }
    }
}
