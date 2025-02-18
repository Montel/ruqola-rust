/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Reaction {
    pub reaction_name: String,
    pub user_names: Vec<String>,
}

impl Default for Reaction {
    fn default() -> Self {
        Reaction::new()
    }
}

impl Reaction {
    pub fn new() -> Self {
        Reaction {
            reaction_name: String::new(),
            user_names: Vec::new(),
        }
    }
}
