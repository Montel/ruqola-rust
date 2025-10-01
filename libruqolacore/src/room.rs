/*
* SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/

use serde::Deserialize;

use crate::messages::Messages;

#[derive(Default, Deserialize, Clone, Debug, PartialEq)]
pub enum RoomType {
    #[serde(alias = "d")]
    Direct,
    #[serde(alias = "c")]
    Channel,
    #[serde(alias = "p")]
    Private,
    #[default]
    Unknown,
}

#[derive(Clone, Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct Room {
    #[serde(rename = "rid")]
    pub room_id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub announcement: String,
    #[serde(default)]
    pub topic: String,
    #[serde(rename = "t")]
    #[serde(default = "roomtype_unknown")]
    pub channel_type: RoomType,
    #[serde(default)]
    pub unread: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub fname: String,
    #[serde(default)] // Return false by default
    pub archived: bool,
    #[serde(default)] // Return false by default
    pub blocker: bool,
    #[serde(default)] // Return false by default
    pub blocked: bool,

    #[serde(skip_deserializing)]
    pub messages: Messages,
    #[serde(default)] // Return false by default
    #[serde(rename = "ro")]
    pub read_only: bool,
    #[serde(default)] // Return false by default
    pub favorite: bool,
    #[serde(default)] // Return false by default
    pub open: bool,

    #[serde(default)] // Return false by default
    pub was_initialized: bool,
}

fn roomtype_unknown() -> RoomType {
    RoomType::Unknown
}
fn convert_str_to_room_type(val: &str) -> RoomType {
    match val {
        "d" => RoomType::Direct,
        "c" => RoomType::Channel,
        "p" => RoomType::Private,
        _ => RoomType::Unknown,
    }
}

impl Room {
    pub fn new() -> Self {
        Room {
            ..Default::default()
        }
    }

    pub fn parse_subscription_room(&mut self, json: &serde_json::Value) {
        // TODO
    }

    pub fn parse_update_room(&mut self, json: &serde_json::Value) {
        if let Some(j) = json.as_object() {
            if j.contains_key("rid") {
                self.room_id = j["rid"].to_string();
            }
        }
    }

    pub fn parse_insert_room(&mut self, json: &serde_json::Value) {
        if let Some(j) = json.as_object() {
            self.room_id = j["_id"].as_str().unwrap().to_string();

            if j.contains_key("announcement") {
                self.announcement = j["announcement"].as_str().unwrap().to_string();
            }
            if j.contains_key("description") {
                self.description = j["description"].as_str().unwrap().to_string();
            }
            if j.contains_key("name") {
                self.name = j["name"].as_str().unwrap().to_string();
            }
            if j.contains_key("fname") {
                if let Some(fname) = j["fname"].as_str() {
                    self.fname = fname.to_string();
                }
            }
            if j.contains_key("ro") {
                self.read_only = j["ro"].as_bool().expect("Invalid json: ro is undefined");
            }
            if j.contains_key("f") {
                self.read_only = j["f"].as_bool().expect("Invalid json: f is undefined");
            }
            if j.contains_key("open") {
                self.read_only = j["open"]
                    .as_bool()
                    .expect("Invalid json: open is undefined");
            }
            if j.contains_key("unread") {
                self.unread = j["unread"]
                    .as_i64()
                    .expect("Invalid json: unread is undefined");
            }
            if j.contains_key("t") {
                self.channel_type = convert_str_to_room_type(j["t"].as_str().unwrap());
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        !self.room_id.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::room::{Room, RoomType};

    #[test]
    fn test_is_empty() {
        let b = Room::new();
        assert!(b.room_id.is_empty());
        assert!(b.description.is_empty());
        assert!(b.announcement.is_empty());
        assert!(b.topic.is_empty());
        assert_eq!(b.unread, 0);
        assert!(b.name.is_empty());
        assert!(b.fname.is_empty());
        assert!(!b.archived);
        assert!(!b.blocker);
        assert!(!b.blocked);
        assert!(!b.read_only);
        assert!(!b.open);
        assert!(!b.favorite);
        assert_eq!(b.channel_type, RoomType::Unknown);
    }
}
