/*
* SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/

#[derive(Clone, Debug, PartialEq)]
pub struct Role {
    pub user_id: String,
    pub user_name: String,
    pub is_moderator: bool,
    pub is_leader: bool,
    pub is_owner: bool,
}

impl Default for Role {
    fn default() -> Self {
        Role::new()
    }
}

impl Role {
    pub fn new() -> Self {
        Role {
            user_id: String::default(),
            user_name: String::default(),
            is_moderator: false,
            is_leader: false,
            is_owner: false,
        }
    }
    pub fn is_valid(self) -> bool {
        !self.user_id.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::roles::role::Role;
    #[test]
    fn test_default_values() {
        let r = Role::default();
        assert!(!r.is_leader);
        assert!(!r.is_moderator);
        assert!(!r.is_owner);
        assert!(r.user_id.is_empty());
        assert!(r.user_name.is_empty());
        assert!(!r.is_valid());
    }
}
