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

impl Default for CustomUserStatusModel {
    fn default() -> Self {
        CustomUserStatusModel::new()
    }
}

impl CustomUserStatusModel {
    fn new() -> Self {
        CustomUserStatusModel {
            custom_user_count: 0,
            offset: 0,
            total: 0,
            statuses: Vec::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::status::customuserstatusmodel::CustomUserStatusModel;

    #[test]
    fn test_is_invalid_by_default() {
        let b: CustomUserStatusModel = CustomUserStatusModel::new();

        assert_eq!(b.total, 0);
        assert_eq!(b.offset, 0);
        assert_eq!(b.custom_user_count, 0);
        assert!(b.statuses.is_empty());
    }
}
// TODO
