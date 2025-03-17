/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::user::Status;
use serde::Deserialize;
#[derive(Clone, Deserialize, Debug)]
pub struct CustomUserStatus {
    #[serde(rename = "_id")]
    pub identifier: String,
    pub name: String,
    #[serde(rename = "statusType")]
    pub status_type: Status,
    // TODO _updatedAt
}

impl Default for CustomUserStatus {
    fn default() -> Self {
        CustomUserStatus::new()
    }
}

impl CustomUserStatus {
    fn new() -> Self {
        CustomUserStatus {
            identifier: String::default(),
            name: String::default(),
            status_type: Status::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{status::customuserstatus::CustomUserStatus, user::Status};

    #[test]
    fn test_is_invalid_by_default() {
        let b: CustomUserStatus = CustomUserStatus::new();

        assert!(b.identifier.is_empty());
        assert!(b.name.is_empty());
        assert_eq!(b.status_type, Status::Unknown);
    }
}
