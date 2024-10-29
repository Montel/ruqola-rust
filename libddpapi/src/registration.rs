/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use crate::rocketchatmessage;
use serde_json::json;
use std::ops::Add;

pub fn registration_list(user_id: String, identifier: &mut u64) -> Vec<String> {
    let mut list = Vec::<String>::new();
    // TODO fix identifier

    let list_stream_notify_user = [
        "/notification",
        "/rooms-changed",
        "/webrtc",
        "/otr",
        "/message",
        "/subscriptions-changed",
    ];
    list_stream_notify_user.iter().for_each(|v| {
        *identifier = identifier.add(1);
        list.push(generate_notification_str(
            String::from("stream-notify-logged"),
            user_id.clone() + &v,
            *identifier,
        ));
    });

    let list_stream_notify_logged = [
        "updateAvatar",
        "roles-change",
        "updateEmojiCustom",
        "deleteEmojiCustom",
        "Users:NameChanged",
        "Users:Deleted",
        "banner-changed",
        "deleteCustomUserStatus",
        "updateCustomUserStatus",
        "voip.statuschanged",
        "user-status",
        "permissions-changed",
        "private-settings-changed",
    ];
    list_stream_notify_logged.iter().for_each(|v| {
        *identifier = identifier.add(1);
        list.push(generate_notification_str(
            String::from("stream-notify-logged"),
            v.to_string(),
            *identifier,
        ));
    });

    let list_stream_notication_all = [
        "public-info",
        "deleteCustomSound",
        "updateCustomSound",
        "deleteEmojiCustom",
        "public-settings-changed",
        "permissions-changed",
        "license",
    ];

    list_stream_notication_all.iter().for_each(|v| {
        *identifier = identifier.add(1);
        list.push(generate_notification_str(
            String::from("stream-notify-all"),
            v.to_string(),
            *identifier,
        ));
    });

    let list_stream_roles = ["roles"];

    list_stream_roles.iter().for_each(|v| {
        *identifier = identifier.add(1);
        list.push(generate_notification_str(
            String::from("stream-roles"),
            v.to_string(),
            *identifier,
        ));
    });

    list
}

// Normal that identifier is not changed because it changed in method which calls it
pub fn generate_notification_str(method_name: String, params: String, identifier: u64) -> String {
    let value = json!(params);
    rocketchatmessage::subscribe(method_name, value, identifier)
}

pub fn registration_room_list(room_id: String, identifier: &mut u64) -> Vec<String> {
    let mut list = Vec::<String>::new();
    let list_params = ["deleteMessage", "deleteMessageBulk", "user-activity"];

    *identifier = identifier.add(1);
    list.push(generate_notification_str(
        room_id.clone(),
        String::from("stream-room-messages"),
        *identifier,
    ));
    *identifier = identifier.add(1);
    list_params.iter().for_each(|v| {
        list.push(generate_notification_str(
            room_id.clone() + "/" + &v,
            String::from("stream-notify-room"),
            *identifier,
        ));
    });

    list
}

#[cfg(test)]
mod tests {
    use crate::registration;
    #[test]
    fn test_generate_notification_str() {
        assert_eq!(
            registration::generate_notification_str(
                String::from("stream-notify-user"),
                String::from("H7Q9djXQ4iShzD9T2/webrtc"),
                5
            ),
            r#"{"id":"5","msg":"sub","name":"stream-notify-user","params":["H7Q9djXQ4iShzD9T2/webrtc",{"args":[],"useCollection":false}]}"#
        );
    }

    #[test]
    fn test_registration_list() {
        let mut value = 5_u64;
        assert_eq!(
            registration::registration_list(String::from("userid"), &mut value).len(),
            27
        );
    }

    #[test]
    fn test_registration_room_list_str() {
        let mut value = 5_u64;
        let list = registration::registration_room_list(String::from("roomId"), &mut value);
        assert_eq!(list.len(), 4);
        assert_eq!(
            list.first().unwrap(),
            r#"{"id":"6","msg":"sub","name":"roomId","params":["stream-room-messages",{"args":[],"useCollection":false}]}"#
        );
    }
}
