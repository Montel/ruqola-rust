/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use std::fmt;

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

/*
Debug output for Replies
*/
impl fmt::Display for Replies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Replies(replies: {:?})", self.replies)
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
