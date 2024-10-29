/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use std::fmt::Display;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MessageReceivedType {
    PingMessage,
    ConnectedMessage,
    ElementAdded(AddElementType),
    ElementChanged(ChangeElementType),
    ElementRemoved(RemoveElementType),
    NoSub(NoSubInfo),
    Ready(String),
    Error(String),
    Unknown(String),
    Result(serde_json::Value),
    InvalidText,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NoSubInfo {
    error: String,
    message: String,
    // FIXME add more info.
    // reason: String,
}

impl Display for NoSubInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error {}\n message {}\n ",
            self.error, self.message /* , self.reason*/
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RemoveElementType {
    User(UserRemoved),
    StreamNotifyLogged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserRemoved {
    pub identifier: String,
}

impl Display for UserRemoved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "identifier {}", self.identifier)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChangeElementType {
    User,
    Rooms,
    StreamRoomMessage(serde_json::Value),
    StreamNotifyUserOtr(serde_json::Value),
    StreamNotifyUserNotification(serde_json::Value),
    StreamNotifyUserWebrtc(serde_json::Value),
    StreamNotifyUserMessage(serde_json::Value),
    StreamNotifyUserUserData(serde_json::Value),
    StreamNotifyUserRoomsChanged(serde_json::Value),
    StreamNotifyUserVideoConference(serde_json::Value),
    StreamNotifyUserSubscriptionsChanged(serde_json::Value),
    StreamNotifyUserForceLogout(serde_json::Value),
    StreamNotifyUserUnknown(serde_json::Value),
    StreamNotifyRoomDeleteMessage,
    StreamNotifyRoomUserActivity,
    StreamNotifyLogged,
    StreamNotifyRoomUnknown,
    StreamNotifyAllUnknown,
    StreamNotifyAllDeleteCustomSound(serde_json::Value),
    StreamNotifyAllUpdateCustomSound(serde_json::Value),
    StreamStout(serde_json::Value),
    RolesChange(serde_json::Value),
    UpdateAvatar(serde_json::Value),
    UpdateEmojiCustom(serde_json::Value),
    UsersNameChanged(serde_json::Value),
    UserDeleted(serde_json::Value),
    DeleteCustomUserStatus(serde_json::Value),
    UpdateCustomUserStatus(serde_json::Value),
    UserStatus(serde_json::Value),
    DeleteEmojiCustom(serde_json::Value),
    PermissionsChanged(serde_json::Value),
    PrivateSettingsChanged(serde_json::Value),
    StreamRoles(serde_json::Value),
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UserAdded {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AddElementType {
    User(UserAdded), // Add arguments
    StreamNotifyUser,
    AutocompleteRecords,
    RoomFiles,
    Unknown,
}

pub fn parse_no_sub_info(json: serde_json::Value) -> NoSubInfo {
    let value = json[r#"error"#].as_object().unwrap();

    NoSubInfo {
        error: String::from(value["error"].as_str().unwrap()),
        message: String::from(value["message"].as_str().unwrap()),
        //reason: String::from(value["reason"].as_str().unwrap()),
    }
}

pub fn parse_received_message(result: Message) -> MessageReceivedType {
    if result.is_text() {
        let text = result.clone().into_text().unwrap();
        let value: serde_json::Value = serde_json::from_str(text.as_str()).unwrap();
        println!("{:?}", value);
        let msg_value = value["msg"].as_str();
        println!("{:?}", msg_value);
        match msg_value {
            Some("ping") => MessageReceivedType::PingMessage,
            Some("connected") => MessageReceivedType::ConnectedMessage,
            Some("added") => MessageReceivedType::ElementAdded(parse_added_element(value)),
            Some("changed") => MessageReceivedType::ElementChanged(parse_changed_element(value)),
            Some("ready") => MessageReceivedType::Ready(text),
            Some("removed") => MessageReceivedType::ElementRemoved(parse_remove_element(value)),
            Some("nosub") => MessageReceivedType::NoSub(parse_no_sub_info(value)),
            Some("result") => MessageReceivedType::Result(value),
            Some("error") => {
                MessageReceivedType::Error(String::from(value["error"].as_str().unwrap()))
            }
            _ => MessageReceivedType::Unknown(text),
        }
    } else {
        MessageReceivedType::InvalidText
    }
}

/// Parse remove element.
/// Remove User id otherwise RemoveElementType::Unknown
fn parse_remove_element(json: serde_json::Value) -> RemoveElementType {
    let collection = json["collection"].as_str();
    if collection.is_none() {
        return RemoveElementType::Unknown;
    }
    match collection.unwrap() {
        "users" => RemoveElementType::User(UserRemoved {
            identifier: String::from(json["id"].as_str().unwrap()),
        }),
        "stream-notify-logged" => RemoveElementType::StreamNotifyLogged,
        _ => RemoveElementType::Unknown,
    }
}

fn parse_changed_element(json: serde_json::Value) -> ChangeElementType {
    let collection = json["collection"].as_str();
    if collection.is_none() {
        return ChangeElementType::Unknown;
    }
    let fields = json["fields"].as_object();
    if fields.is_none() {
        // Report error.
        return ChangeElementType::Unknown;
    }

    match collection.unwrap() {
        "stream-room-messages" => {
            let contents = fields.unwrap()["args"].to_owned();
            ChangeElementType::StreamRoomMessage(contents)
        }
        "rooms" => ChangeElementType::Rooms,
        "users" => ChangeElementType::User,
        "stream-notify-user" => {
            let event_name = fields.unwrap()["eventName"].as_str();
            if event_name.is_none() {
                return ChangeElementType::Unknown;
            }
            let event_name = event_name.unwrap();
            let contents = fields.unwrap()["args"].to_owned();
            if event_name.ends_with("/subscriptions-changed") {
                ChangeElementType::StreamNotifyUserSubscriptionsChanged(contents)
            } else if event_name.ends_with("/rooms-changed") {
                ChangeElementType::StreamNotifyUserRoomsChanged(contents)
            } else if event_name.ends_with("/notification") {
                ChangeElementType::StreamNotifyUserNotification(contents)
            } else if event_name.ends_with("/webrtc") {
                ChangeElementType::StreamNotifyUserWebrtc(contents)
            } else if event_name.ends_with("/otr") {
                ChangeElementType::StreamNotifyUserOtr(contents)
            } else if event_name.ends_with("/message") {
                ChangeElementType::StreamNotifyUserMessage(contents)
            } else if event_name.ends_with("/userData") {
                ChangeElementType::StreamNotifyUserUserData(contents)
            } else if event_name.ends_with("/video-conference") {
                ChangeElementType::StreamNotifyUserVideoConference(contents)
            } else if event_name.ends_with("/force_logout") {
                ChangeElementType::StreamNotifyUserForceLogout(contents)
            } else {
                ChangeElementType::StreamNotifyUserUnknown(contents)
            }
        }
        "stream-notify-room" => {
            let event_name = fields.unwrap()["eventName"].as_str();
            if event_name.is_none() {
                return ChangeElementType::Unknown;
            }
            let event_name = event_name.unwrap();
            if event_name.ends_with("/deleteMessage") {
                // TODO roomId and messageId
                ChangeElementType::StreamNotifyRoomDeleteMessage
            } else if event_name.ends_with("user-activity") {
                // TODO parse element
                ChangeElementType::StreamNotifyRoomUserActivity
            } else {
                ChangeElementType::StreamNotifyRoomUnknown
            }
        }
        "stream-notify-logged" => {
            let event_name = fields.unwrap()["eventName"].as_str();
            if event_name.is_none() {
                return ChangeElementType::Unknown;
            }
            let contents = fields.unwrap()["args"].to_owned();
            match event_name.unwrap() {
                "roles-change" => ChangeElementType::RolesChange(contents),
                "updateAvatar" => ChangeElementType::UpdateAvatar(contents),
                "updateEmojiCustom" => ChangeElementType::UpdateEmojiCustom(contents),
                "Users:NameChanged" => ChangeElementType::UsersNameChanged(contents),
                "Users:Deleted" => ChangeElementType::UserDeleted(contents),
                "deleteCustomUserStatus" => ChangeElementType::DeleteCustomUserStatus(contents),
                "updateCustomUserStatus" => ChangeElementType::UpdateCustomUserStatus(contents),
                "user-status" => ChangeElementType::UserStatus(contents),
                "deleteEmojiCustom" => ChangeElementType::DeleteEmojiCustom(contents),
                "permissions-changed" => ChangeElementType::PermissionsChanged(contents),
                "private-settings-changed" => ChangeElementType::PrivateSettingsChanged(contents),
                _ => ChangeElementType::StreamNotifyLogged,
            }
        }
        "stream-notify-all" => {
            let event_name = fields.unwrap()["eventName"].as_str();
            if event_name.is_none() {
                return ChangeElementType::Unknown;
            }
            let contents = fields.unwrap()["args"].to_owned();
            match event_name.unwrap() {
                "deleteCustomSound" => {
                    ChangeElementType::StreamNotifyAllDeleteCustomSound(contents)
                }
                "updateCustomSound" => {
                    ChangeElementType::StreamNotifyAllUpdateCustomSound(contents)
                }
                _ => ChangeElementType::StreamNotifyAllUnknown,
            }
        }
        "stream-stdout" => {
            let contents = fields.unwrap()["args"].to_owned();
            ChangeElementType::StreamStout(contents)
        }
        "stream-roles" => {
            let event_name = fields.unwrap()["eventName"].as_str();
            if event_name.is_none() {
                return ChangeElementType::Unknown;
            }
            if event_name.unwrap() == "roles" {
                let contents = fields.unwrap()["args"].to_owned();
                return ChangeElementType::StreamRoles(contents);
            }
            ChangeElementType::Unknown
        }
        _ => ChangeElementType::Unknown,
    }
}

fn parse_added_element(json: serde_json::Value) -> AddElementType {
    let collection = json["collection"].as_str();
    if collection.is_none() {
        return AddElementType::Unknown;
    }
    let fields = json["fields"].as_object();
    if fields.is_none() {
        // Report error.
        return AddElementType::Unknown;
    }

    match collection.unwrap() {
        "users" => AddElementType::User(UserAdded {
            user_id: String::from(json["id"].as_str().unwrap()),
            user_name: String::from(fields.unwrap()["username"].as_str().unwrap()),
        }),
        "autocompleteRecords" => AddElementType::AutocompleteRecords,
        "room_files" => AddElementType::RoomFiles, // TODO implement it.
        _ => AddElementType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use crate::ddpmessage;
    use crate::ddpmessage::ChangeElementType;
    use serde_json::json;
    use tokio_tungstenite::tungstenite::protocol::Message;
    #[test]
    fn test_parse_received_message() {
        // Ping message
        {
            let request = r#"{"msg": "ping"}"#;
            let message = Message::Text(request.to_string());
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::PingMessage
            );
        }
        // Invalid message
        {
            let request: &str = r#"{"msg": "foo"}"#;
            let message = Message::Text(request.to_string());
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::Unknown(request.to_string())
            );
        }
        // Connected message
        {
            let request: &str = r#"{"msg": "connected"}"#;
            let message = Message::Text(request.to_string());
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::ConnectedMessage
            );
        }
        // Ready message
        {
            let request: &str = r#"{"msg": "ready"}"#;
            let message = Message::Text(request.to_string());
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::Ready(request.to_string())
            );
        }
        // Error message
        {
            let request: &str = r#"{"msg": "error", "error": "invalid"}"#;
            let message = Message::Text(request.to_string());
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::Error("invalid".to_string())
            );
        }
        // change
        {
            let request: &str = r#"{"collection":"stream-notify-logged","fields":{"args":[["hLdKM5PH","foo",0,"",null,["user"]]],"eventName":"user-status"},"id":"id","msg":"changed"}"#;
            let message = Message::Text(request.to_string());
            let value = json!([["hLdKM5PH", "foo", 0, "", null, ["user"]]]);
            assert_eq!(
                ddpmessage::parse_received_message(message),
                ddpmessage::MessageReceivedType::ElementChanged(ChangeElementType::UserStatus(
                    value
                ))
            );
        }
    }
}
