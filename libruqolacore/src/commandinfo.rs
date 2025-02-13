/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(default)]
pub struct CommandInfo {
    pub command: String,
    pub params: String,
    pub description: String,
}

impl CommandInfo {
    pub fn new() -> Self {
        CommandInfo {
            ..Default::default()
        }
    }
}

/*
Debug output for CommandInfo
TODO verify it
*/
impl fmt::Display for CommandInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CommandInfo command: {}, params: {}, description {})",
            self.command, self.params, self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::commandinfo::CommandInfo;
    use std::fs::File;

    pub fn parse(filename: &str) -> CommandInfo {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }
    #[test]
    fn test_commandinfo_default() {
        let command_info = CommandInfo::new();
        assert!(command_info.command.is_empty());
        assert!(command_info.params.is_empty());
        assert!(command_info.description.is_empty());
    }
}
