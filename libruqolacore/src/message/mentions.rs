/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
#[derive(Clone, Default, Deserialize, Debug)]
pub struct Mentions {
    pub mentions: HashMap<String, String>,
}

impl Mentions {
    pub fn new() -> Self {
        Mentions {
            mentions: HashMap::default(),
        }
    }
}

/*
Debug output for Mentions
*/
impl fmt::Display for Mentions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mentions(mentions: {:?})", self.mentions)
    }
}

#[cfg(test)]
mod tests {
    use crate::message::mentions::Mentions;

    #[test]
    fn test_is_empty() {
        let b = Mentions::new();
        assert!(b.mentions.is_empty());
    }
}
