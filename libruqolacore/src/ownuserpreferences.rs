/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub enum RoomListSortOrder {
    #[default]
    Unknown,
    #[serde(alias = "activity")]
    ByLastMessage,
    #[serde(alias = "alphabetical")]
    Alphabetically,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Default)]
pub enum RoomListDisplay {
    #[default]
    Unknown,
    #[serde(alias = "condensed")]
    Condensed,
    #[serde(alias = "medium")]
    Medium,
    #[serde(alias = "extended")]
    Extended,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct OwnUserPreferences {
    #[serde(rename = "highlights")]
    pub highlight_words: Vec<String>,
    #[serde(rename = "emailNotificationMode")]
    pub email_notification_mode: String,
    #[serde(rename = "desktopNotifications")]
    pub desktop_notifications: String,
    #[serde(default)]
    #[serde(rename = "pushNotifications")]
    pub push_notifications: String,
    #[serde(rename = "newMessageNotification")]
    pub new_message_notification: String,
    #[serde(rename = "newRoomNotification")]
    pub new_room_notification: String,
    #[serde(rename = "sidebarSortby")]
    pub room_list_sort_order: RoomListSortOrder,
    #[serde(rename = "sidebarViewMode")]
    pub room_list_display: RoomListDisplay,

    #[serde(rename = "idleTimeLimit")]
    pub idle_time_limit: i64, // -1 TOTO
    #[serde(rename = "notificationsSoundVolume")]
    pub notifications_sound_volume: i64, // -1 TODO
    pub convert_ascii_emoji: bool,
    // Fix default value pub mUseEmojis = true;
    pub hide_roles: bool,
    // Fix default value pub mDisplayAvatars = true;
    pub enable_auto_away: bool,
    pub show_unread: bool,
    pub show_room_avata: bool,
    pub show_favorite: bool,
    pub receive_login_detection_email: bool,
    pub mute_focused_conversations: bool,
}

impl Default for OwnUserPreferences {
    fn default() -> Self {
        OwnUserPreferences::new()
    }
}

impl OwnUserPreferences {
    pub fn new() -> Self {
        OwnUserPreferences {
            // TODO
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ownuserpreferences::OwnUserPreferences;
    use std::fs::File;

    // Using by test
    pub fn parse(filename: &str) -> OwnUserPreferences {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_parsing() {
        {
            // Load file
            let preferences = parse("src/data/ownuserpreferences/ownuserpreferences1.json");
            //assert_eq!(permission.update.len(), 1147);
            //assert_eq!(permission.remove.len(), 0);
        }
    }
}
