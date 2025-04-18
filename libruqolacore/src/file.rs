/*
 * SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;

#[derive(Clone, Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct File {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub description: String,
    #[serde(rename = "name")]
    pub file_name: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub url: String,
    pub path: String,
    #[serde(rename = "typeGroup")]
    pub type_group: String,
    #[serde(rename = "_id")]
    /// In rest api we use it.
    pub file_identifier: String,
    #[serde(rename = "rid")]
    pub room_id: String,
    // TODO uploadedAt + user
}

impl File {
    pub fn new() -> Self {
        File {
            ..Default::default()
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(userId {}, description {}, name {}, mimetype {}, url {}, type_group {}, identifier {}, path {}, rid {})"
         , self.user_id, self.description, self.file_name, self.mime_type, self.url, self.type_group, self.file_identifier, self.path, self.room_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::file::File;

    #[test]
    fn test_is_empty() {
        let b = File::new();
        assert!(b.user_id.is_empty());
        assert!(b.description.is_empty());
        assert!(b.file_name.is_empty());
        assert!(b.mime_type.is_empty());
        assert!(b.url.is_empty());
        assert!(b.type_group.is_empty());
        assert!(b.file_identifier.is_empty());
        assert!(b.path.is_empty());
        assert!(b.room_id.is_empty());
    }
}
