/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use std::collections::HashMap;

use serde::Deserialize;
#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub struct MessageTranslations {
    #[serde(rename = "translations")]
    pub translated_string: HashMap<String, String>,
}

// TODO add test
