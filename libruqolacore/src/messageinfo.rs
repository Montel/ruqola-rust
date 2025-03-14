/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;
use std::fmt;
#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub enum SystemMessageType {
    #[default]
    Unknown,
    #[serde(alias = "uj")]
    UserJoined,
    #[serde(alias = "ul")]
    UserLeft,
    #[serde(alias = "ult")]
    UserLeftTeam,
    #[serde(alias = "room_changed_topic")]
    RoomTopicChanged,
    #[serde(alias = "au")]
    UserAdded,
    #[serde(alias = "r")]
    RoomNameChanged,
    #[serde(alias = "ru")]
    UserRemoved,
    #[serde(alias = "room_changed_description")]
    RoomDescriptionChanged,
    #[serde(alias = "room_changed_announcement")]
    RoomAnnoucementChanged,
    #[serde(alias = "room_changed_privacy")]
    RoomPrivacyChanged,
    #[serde(alias = "jitsi_call_started")]
    JitsiCallStarted,
    #[serde(alias = "rm")]
    MessageDeleted,
    #[serde(alias = "message_pinned")]
    Pinned,
    #[serde(alias = "otr")]
    EncryptedMessage,
    #[serde(alias = "user-muted")]
    UserMuted,
    #[serde(alias = "user-unmuted")]
    UserUnmuted,
    #[serde(alias = "subscription-role-added")]
    SubscriptionRoleAdded,
    #[serde(alias = "subscription-role-removed")]
    SubscriptionRoleRemoved,
    #[serde(alias = "e2e")]
    MessageE2E,
    #[serde(alias = "discussion-created")]
    DiscussionCreated,
    #[serde(alias = "ut")]
    UserJoinedConversation,
    #[serde(alias = "room-archived")]
    RoomArchived,
    #[serde(alias = "room-unarchived")]
    RoomUnarchived,
    #[serde(alias = "rtc")]
    Rtc,
    #[serde(alias = "wm")]
    Welcome,
    #[serde(alias = "room_changed_avatar")]
    RoomAvatarChanged,
    #[serde(alias = "room_e2e_enabled")]
    RoomE2eEnabled,
    #[serde(alias = "room_e2e_disabled")]
    RoomE2eDisabled,
    #[serde(alias = "room-set-read-only")]
    RoomSetReadOnly,
    #[serde(alias = "room-removed-read-only")]
    RoomRemoveReadOnly,
    #[serde(alias = "added-user-to-team")]
    AddedUserToTeam,
    #[serde(alias = "removed-user-from-team")]
    RemovedUserFromTeam,
    #[serde(alias = "user-converted-to-team")]
    UserConvertedToTeam,
    #[serde(alias = "user-converted-to-channel")]
    UserConvertedToChannel,
    #[serde(alias = "user-removed-room-from-team")]
    UserRemovedRoomFromTeam,
    #[serde(alias = "user-deleted-room-from-team")]
    UserDeletedRoomFromTeam,
    #[serde(alias = "user-added-room-to-team")]
    UserAddedRoomToTeam,
    #[serde(alias = "room-allowed-reacting")]
    RoomAllowedReacting,
    #[serde(alias = "room-disallowed-reacting")]
    RoomDisallowedReacting,
    #[serde(alias = "ujt")]
    UserJoinedTeam,
    #[serde(alias = "user_joined_otr")]
    UserJoinedOtr,
    #[serde(alias = "user_key_refreshed_successfully")]
    UserKeyRefreshedSuccessfully,
    #[serde(alias = "user_requested_otr_key_refresh")]
    UserRequesterOtrKeyRefresh,
    #[serde(alias = "videoconf")]
    VideoConf,
}

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessageInfo {
    #[serde(rename = "_id")]
    pub identifier: String,
    #[serde(rename = "msg")]
    pub message: String,
    pub alias: String,
    pub emoji: String,
    pub avatar: String,
    #[serde(default)]
    pub tmid: String,
    #[serde(default)]
    pub drid: String,
    pub role: String,
    #[serde(default)]
    pub tcount: i64,
    #[serde(default)]
    pub dcount: i64,
    // TODO add type
    pub rid: String,

    pub unread: bool,
}

impl Default for MessageInfo {
    fn default() -> Self {
        MessageInfo::new()
    }
}

impl MessageInfo {
    pub fn new() -> Self {
        MessageInfo {
            identifier: String::default(),
            message: String::default(),
            alias: String::default(),
            emoji: String::default(),
            avatar: String::default(),
            tmid: String::default(),
            drid: String::default(),
            role: String::default(),
            rid: String::default(),
            tcount: 0,
            dcount: 0,
            unread: false,
        }
    }
    pub fn parse(filename: &str) -> MessageInfo {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

/*
Debug output for MessageInfo
*/
impl fmt::Display for MessageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(message identifier: {}, message: {}, alias: {})",
            self.identifier, self.message, self.alias
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::messageinfo::MessageInfo;

    #[test]
    fn test_is_empty() {
        let b = MessageInfo::new();
        assert!(b.identifier.is_empty());
        assert!(b.alias.is_empty());
        assert!(b.emoji.is_empty());
        assert!(b.avatar.is_empty());
        assert!(b.tmid.is_empty());
        assert!(b.drid.is_empty());
        assert!(b.role.is_empty());
        assert_eq!(b.dcount, 0);
        assert_eq!(b.tcount, 0);
        assert!(b.rid.is_empty());
    }
}
