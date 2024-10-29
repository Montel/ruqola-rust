/*
 * SPDX-FileCopyrightText:  2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Default, Deserialize, Clone, Debug, PartialEq)]
pub enum Status {
    #[serde(alias = "online")]
    Online,
    #[serde(alias = "busy")]
    Busy,
    #[serde(alias = "away")]
    Away,
    #[serde(alias = "offline")]
    Offline,
    #[default]
    Unknown,
}

#[derive(Clone, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct UserEmailsInfo {
    pub address: String,
    pub verified: bool,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(default)]
pub struct User {
    #[serde(rename = "_id")]
    pub user_id: String,
    pub name: String,
    pub username: String,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub bio: String,
    #[serde(rename = "nickname")]
    pub nick_name: String,
    pub active: bool,
    pub roles: Vec<String>,
    pub status: Status,
    #[serde(rename = "utcOffset")]
    pub utc_offset: f32,
    pub emails: Vec<UserEmailsInfo>,
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}

impl User {
    pub fn new() -> Self {
        User {
            user_id: String::new(),
            name: String::new(),
            username: String::new(),
            status_text: String::new(),
            bio: String::new(),
            nick_name: String::new(),
            active: false,
            roles: Vec::<String>::default(),
            status: Status::Unknown,
            utc_offset: 0.0,
            emails: Vec::<UserEmailsInfo>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<User>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::{Status, User, UserEmailsInfo};
    use std::fs::File;
    pub fn parse(filename: &str) -> User {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }
    #[test]
    fn test_is_empty() {
        let b = User::new();
        assert!(b.user_id.is_empty());
        assert!(b.name.is_empty());
        assert!(b.username.is_empty());
        assert!(b.status_text.is_empty());
        assert!(b.bio.is_empty());
        assert!(b.nick_name.is_empty());
        assert!(!b.active);
        assert!(b.roles.is_empty());
        assert_eq!(b.utc_offset, 0.0);
    }

    #[test]
    fn test_parse_data() {
        {
            let b = parse("src/data/user/userrestapi.json");
            assert_eq!(b.name, "name_user");
            assert_eq!(b.user_id, "BDFj6E7Z9RYucn8C");
            assert_eq!(b.username, "username");
            let roles = vec![r#"user"#];
            assert_eq!(b.roles, roles);
            assert_eq!(b.status, Status::Offline);
            assert_eq!(b.utc_offset, 0.0);

            // QDateTime createdTime;
            // createdTime.setDate(QDate(2020, 10, 04));
            // createdTime.setTime(QTime(22, 48, 01, 903));
            // createdTime.setTimeZone(QTimeZone::UTC);
            // expected.setCreatedAt(createdTime);
            // expected.setLastLogin(QDateTime());
        }
        {
            let b = parse("src/data/user/userrestapi2.json");
            assert_eq!(b.name, "Bla bla");
            assert_eq!(b.user_id, "XQZAk3998f9hSNwh");
            assert_eq!(b.username, "steffen");
            let roles = vec![r#"user"#, r#"admin"#];
            assert_eq!(b.roles, roles);
            assert_eq!(b.status, Status::Online);
            assert_eq!(b.utc_offset, 2.0);
            let mut user_emails_info = UserEmailsInfo::default();
            user_emails_info.address = String::from("bla@kde.com");
            user_emails_info.verified = true;
            assert_eq!(b.emails, vec![user_emails_info]);
            assert_eq!(b.nick_name, "TheReal");

            // User expected;
            // QDateTime createdTime;
            // createdTime.setTimeZone(QTimeZone::UTC);
            // createdTime.setDate(QDate(2018, 01, 18));
            // createdTime.setTime(QTime(11, 52, 50, 772));

            // expected.setCreatedAt(createdTime);
            // QDateTime lastLogin;
            // lastLogin.setTimeZone(QTimeZone::UTC);
            // lastLogin.setDate(QDate(2020, 10, 12));
            // lastLogin.setTime(QTime(00, 26, 27, 324));
            // expected.setLastLogin(lastLogin);
        }
    }
}
