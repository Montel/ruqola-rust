/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::invitation::invitation::Invitation;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Invitations {
    pub invitations: Vec<Invitation>,
}

impl Default for Invitations {
    fn default() -> Self {
        Invitations::new()
    }
}

impl Invitations {
    pub fn new() -> Self {
        Invitations {
            invitations: Vec::<Invitation>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<Invitations>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::invitation::invitations::Invitations;

    // For test !
    pub fn parse(filename: &str) -> Invitations {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_empty() {
        let b = Invitations::new();
        assert!(b.invitations.is_empty());
    }
}
