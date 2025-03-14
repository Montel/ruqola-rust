/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */


use std::fmt;
#[derive(Default, Clone, Debug, PartialEq)]
pub enum NotificationType {
    #[default]
    StandardMessage,
    ConferenceCall,
}
// TODO implement display for enum

pub struct NotificationInfo {
    pub message_id: String,
    pub account_name: String,
    pub message: String,
    pub title: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_user_name: String,
    pub room_name: String,
    pub room_id: String,
    pub channel_type: String,
    pub tm_id: String,
    pub date_time: String,
    pub notification_type: NotificationType,
}

/*
Debug output for MessageInfo
*/
impl fmt::Display for NotificationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(message identifier: {}, accountName: {}, message: {}, title: {}, senderId: {}, senderName: {}, senderUserName: {}, roomName: {}, roomId: {}, channelType: {}, mTmId: {}, mDateTime: {}, mNotificationType: {:?})",
            self.message_id, self.account_name, self.message, self.title, self.sender_id, self.sender_name, self.sender_user_name, self.room_name, self.room_id, self.channel_type, self.tm_id, self.date_time, self.notification_type  
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::notificationinfo::NotificationInfo;

    #[test]
    fn test_is_empty() {
        // TODO
        /*
        let b = NotificationInfo::new();
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
        */
    }
}
