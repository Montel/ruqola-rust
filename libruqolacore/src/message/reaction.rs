/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Reaction {
    pub reaction_name: String,
    #[serde(rename = "usernames")]
    pub user_names: Vec<String>,
}

impl Default for Reaction {
    fn default() -> Self {
        Reaction::new()
    }
}

impl Reaction {
    pub fn new() -> Self {
        Reaction {
            reaction_name: String::new(),
            user_names: Vec::new(),
        }
    }

    pub fn converted_users_name_as_tooltip(self) -> String {
        let result = String::from("");
        if self.user_names.is_empty() {
            result
        } else if self.user_names.len() == 1 {
            let initial_value = self.user_names[0].clone();
            let reaction_name = self.reaction_name;
            return format!("{initial_value} reacted with {reaction_name}");
        } else {
            let mut notification_str = String::from("");
            let total = self.user_names.len();
            for i in 0..total {
                let user = self.user_names[i].clone();
                if i == 0 {
                    notification_str = user;
                } else if i < (total - 1) {
                    notification_str = format!("{notification_str}, {user}");
                } else {
                    notification_str = format!("{notification_str} and {user}");
                }
            }
            let reaction_name = self.reaction_name;
            return format!("{notification_str} reacted with {reaction_name}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::reaction::Reaction;

    #[test]
    fn test_tooltip() {
        let mut reaction: Reaction = Reaction::new();
        reaction.reaction_name = ":foo:".to_string();
        assert!(reaction
            .clone()
            .converted_users_name_as_tooltip()
            .is_empty(),);

        reaction.user_names = vec!["bla".to_string()];
        assert_eq!(
            reaction.clone().converted_users_name_as_tooltip(),
            "bla reacted with :foo:"
        );

        reaction.user_names = vec!["bla".to_string(), "blo".to_string()];
        assert_eq!(
            reaction.clone().converted_users_name_as_tooltip(),
            "bla and blo reacted with :foo:"
        );

        reaction.user_names = vec!["bla".to_string(), "blo".to_string(), "bli".to_string()];
        assert_eq!(
            reaction.clone().converted_users_name_as_tooltip(),
            "bla, blo and bli reacted with :foo:"
        );
    }
}
