/*
* SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/

use crate::roles::role::Role;
#[derive(Clone, Debug, PartialEq)]

pub struct Roles {
    pub roles: Vec<Role>,
}

impl Default for Roles {
    fn default() -> Self {
        Roles::new()
    }
}

impl Roles {
    pub fn new() -> Self {
        Roles {
            roles: Vec::<Role>::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::roles::roles::Roles;
    #[test]
    fn test_default_values() {
        // TODO
    }
}
