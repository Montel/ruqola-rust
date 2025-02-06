/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

#[derive(Clone, Debug, PartialEq)]
pub struct LicensesManager {
    pub licenses: Vec<String>,
}

impl Default for LicensesManager {
    fn default() -> Self {
        LicensesManager::new()
    }
}

impl LicensesManager {
    pub fn new() -> Self {
        LicensesManager {
            licenses: Vec::<String>::default(),
        }
    }
}

// TODO add test
