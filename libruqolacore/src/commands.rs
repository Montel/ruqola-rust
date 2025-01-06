/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::commandinfo::CommandInfo;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
#[serde(default)]
pub struct Commands {
    pub commands: Vec<CommandInfo>,
}

impl Default for Commands {
    fn default() -> Self {
        Commands::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Commands {
            commands: Vec::<CommandInfo>::default(),
        }
    }

    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<Commands>(json) {
            *self = val;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::Commands;
    use std::fs::File;
    // For test !
    pub fn parse(filename: &str) -> Commands {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }
    #[test]
    fn test_is_empty() {
        let b = Commands::new();
        assert!(b.commands.is_empty());
    }
    #[test]
    fn test_parse_data() {
        {
            // Load file
            let result = parse("src/data/commands/command2.json");
            let commands = result.commands;
            assert!(!commands.is_empty());
            assert_eq!(commands.len(), 25);
        }
        {
            // Load file
            let result = parse("src/data/commands/command1.json");

            assert!(result.commands.is_empty());
        }
        {
            // Load file
            let result = parse("src/data/commands/command3.json");
            let commands = result.commands;
            assert!(!commands.is_empty());
            assert_eq!(commands.len(), 3);
        }
    }
}
