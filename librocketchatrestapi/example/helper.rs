/*
 * SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub password: String,
    pub username: String,
    pub domain: String,
}

// Modify it.
pub fn default_config_path() -> String {
    r#"/opt/travail/git/ruqola-rust/librocketchatrestapi/example-config.toml"#.to_string()
}
