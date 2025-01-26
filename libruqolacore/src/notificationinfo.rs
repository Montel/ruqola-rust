/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::Deserialize;

use std::fmt;
#[derive(Default, Clone, Debug, PartialEq)]
pub enum NotificationType {
    #[default]
    StandardMessage,
    ConferenceCall,
}
// TODO implement display for enum

pub struct NotificationInfo {
    pub mMessageId: String,
    pub mAccountName: String,
    pub mMessage: String,
    pub mTitle: String,
    pub mSenderId: String,
    pub mSenderName: String,
    pub mSenderUserName: String,
    pub mRoomName: String,
    pub mRoomId: String,
    pub mChannelType: String,
    pub mTmId: String,
    pub mDateTime: String,
    pub mNotificationType: NotificationType,
}

/*
Debug output for MessageInfo
*/
impl fmt::Display for NotificationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(message identifier: {}, accountName: {}, message: {}, title: {}, senderId: {}, senderName: {}, senderUserName: {}, roomName: {}, roomId: {}, channelType: {}, mTmId: {}, mDateTime: {}, mNotificationType: {:?})",
            self.mMessageId, self.mAccountName, self.mMessage, self.mTitle, self.mSenderId, self.mSenderName, self.mSenderUserName, self.mRoomName, self.mRoomId, self.mChannelType, self.mTmId, self.mDateTime, self.mNotificationType  
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
