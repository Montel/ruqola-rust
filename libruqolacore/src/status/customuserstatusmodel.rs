/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

use super::customuserstatus::CustomUserStatus;
#[derive(Clone, Deserialize, Debug)]
pub struct CustomUserStatusModel {
    #[serde(rename = "count")]
    pub custom_user_count: i64,
    pub offset: i64,
    pub total: i64,
    pub statuses: Vec<CustomUserStatus>,
}

// TODO
