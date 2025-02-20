/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AvatarType {
    Unknown,
    Room,
    User,
    UserAndRoom,
}

#[derive(Clone, Debug)]
pub struct AvatarInfo {
    pub etag: String,
    pub identifier: String,
    pub avatar_type: AvatarType,
}

impl Default for AvatarInfo {
    fn default() -> Self {
        AvatarInfo::new()
    }
}

impl AvatarInfo {
    pub fn new() -> Self {
        AvatarInfo {
            etag: String::new(),
            identifier: String::new(),
            avatar_type: AvatarType::Unknown,
        }
    }
    pub fn is_valid(&self) -> bool {
        (self.avatar_type != AvatarType::Unknown) && !self.identifier.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::avatarinfo::{AvatarInfo, AvatarType};

    #[test]
    fn test_default_values() {
        let b = AvatarInfo::new();
        assert!(b.etag.is_empty());
        assert!(b.identifier.is_empty());
        assert_eq!(b.avatar_type, AvatarType::Unknown);
    }
}
