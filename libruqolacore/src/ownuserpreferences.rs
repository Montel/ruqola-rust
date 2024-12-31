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
    #[serde(rename = "pushNotifications")]
    pub push_notifications: String,
    #[serde(rename = "newMessageNotification")]
    pub new_message_notification: String,
    #[serde(rename = "newRoomNotification")]
    pub new_room_notification: String,
    /*
    //TODO
    RoomListSortOrder mRoomListSortOrder = RoomListSortOrder::Unknown;
    RoomListDisplay mRoomListDisplay = RoomListDisplay::Unknown;
    */
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
    use crate::ownuserpreferences;
}
