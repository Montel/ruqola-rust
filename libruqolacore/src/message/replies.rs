/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Replies {
    pub replies: Vec<String>,
}
impl Default for Replies {
    fn default() -> Self {
        Replies::new()
    }
}
impl Replies {
    pub fn new() -> Self {
        Replies {
            replies: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Replies;
    #[test]
    fn test_default_values() {
        let v = Replies::new();
        assert!(v.replies.is_empty());
    }
}
