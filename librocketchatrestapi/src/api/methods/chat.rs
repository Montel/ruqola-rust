/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use crate::api::methods::base::EndPointInfo;
use crate::api::methods::restapiutils::RestApiUrlType;
use async_trait::async_trait;
use reqwest::Method;
use std::collections::HashMap;

use crate::api::methods::{base::PayloadValue, APIMethod};
use libauthenticationbase::authenticationsettings::AuthenticationType;
// PostMessageMethod
#[derive(Debug)]
pub struct PostMessageMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub room_id: String,
    pub text: Option<String>,
    pub alias: Option<String>,
    pub emoji: Option<String>,
    pub avatar: Option<String>,
}

impl Default for PostMessageMethod {
    fn default() -> Self {
        PostMessageMethod {
            settings: AuthenticationType::None,
            room_id: String::default(),
            text: None,
            alias: None,
            emoji: None,
            avatar: None,
            server_url: String::default(),
        }
    }
}

#[async_trait]
impl APIMethod for PostMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatPostMessage,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }
    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("roomId".to_string(), PayloadValue::String(&self.room_id));

        if let Some(text) = &self.text {
            payload.insert("text".to_string(), PayloadValue::String(text));
        }
        if let Some(alias) = &self.alias {
            payload.insert("alias".to_string(), PayloadValue::String(alias));
        }
        if let Some(emoji) = &self.emoji {
            payload.insert("emoji".to_string(), PayloadValue::String(emoji));
        }
        if let Some(avatar) = &self.avatar {
            payload.insert("avatar".to_string(), PayloadValue::String(avatar));
        }

        Some(payload)
    }
}

// DeleteMessageMethod
#[derive(Debug)]
pub struct DeleteMessageMethod {
    pub settings: AuthenticationType,

    pub room_id: String,
    pub message_id: String,
    pub as_user: Option<bool>,
    pub server_url: String,
}

impl Default for DeleteMessageMethod {
    fn default() -> Self {
        DeleteMessageMethod {
            settings: AuthenticationType::None,
            room_id: String::default(),
            message_id: String::default(),
            as_user: None,
            server_url: String::default(),
        }
    }
}

#[async_trait]
impl APIMethod for DeleteMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatDelete,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }
    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("roomId".to_string(), PayloadValue::String(&self.room_id));
        payload.insert("msgId".to_string(), PayloadValue::String(&self.message_id));

        if let Some(as_user) = &self.as_user {
            payload.insert("asUser".to_string(), PayloadValue::Bool(as_user));
        }
        Some(payload)
    }
}

//StarMessageMethod
#[derive(Debug)]
pub struct StarMessageMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub message_id: String,
}

impl Default for StarMessageMethod {
    fn default() -> Self {
        StarMessageMethod {
            settings: AuthenticationType::None,
            message_id: String::default(),
            server_url: String::new(),
        }
    }
}

#[async_trait]
impl APIMethod for StarMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatStarMessage,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert(
            "messageId".to_string(),
            PayloadValue::String(&self.message_id),
        );
        Some(payload)
    }
}

//UnStarMessageMethod
#[derive(Debug)]
pub struct UnStarMessageMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub message_id: String,
}

impl Default for UnStarMessageMethod {
    fn default() -> Self {
        UnStarMessageMethod {
            settings: AuthenticationType::None,
            message_id: String::default(),
            server_url: String::new(),
        }
    }
}

#[async_trait]
impl APIMethod for UnStarMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatUnStarMessage,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert(
            "messageId".to_string(),
            PayloadValue::String(&self.message_id),
        );
        Some(payload)
    }
}

//FollowMessageMethod
#[derive(Debug)]
pub struct FollowMessageMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub mid: String,
}

impl Default for FollowMessageMethod {
    fn default() -> Self {
        FollowMessageMethod {
            settings: AuthenticationType::None,
            mid: String::default(),
            server_url: String::new(),
        }
    }
}

#[async_trait]
impl APIMethod for FollowMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatFollowMessage,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("mid".to_string(), PayloadValue::String(&self.mid));
        Some(payload)
    }
}

//Ignore User
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct IgnoreUserMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub rid: String,
    pub userId: String,
    pub ignore: bool,
}

impl Default for IgnoreUserMethod {
    fn default() -> Self {
        IgnoreUserMethod {
            settings: AuthenticationType::None,
            rid: String::default(),
            userId: String::default(),
            ignore: true,
            server_url: String::default(),
        }
    }
}

#[async_trait]
impl APIMethod for IgnoreUserMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatIgnoreUser,
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("rid".to_string(), PayloadValue::String(&self.rid));
        payload.insert("userId".to_string(), PayloadValue::String(&self.userId));
        if self.ignore {
            payload.insert("ignore".to_string(), PayloadValue::String("true"));
        } else {
            payload.insert("ignore".to_string(), PayloadValue::String("false"));
        }
        Some(payload)
    }
}

//Snippeted Messages
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct SnippetedMessagesMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub roomId: String,
}

impl Default for SnippetedMessagesMethod {
    fn default() -> Self {
        SnippetedMessagesMethod {
            settings: AuthenticationType::None,
            roomId: String::default(),
            server_url: String::default(),
        }
    }
}

#[async_trait]
impl APIMethod for SnippetedMessagesMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChatGetSnippetedMessages,
            ..Default::default()
        }
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("roomId".to_string(), self.roomId.clone());
        Some(payload)
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        None
    }
}

//StarMessageMethod
#[derive(Debug)]
pub struct PinMessageMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub message_id: String,
    pub pin_message: bool,
}

impl Default for PinMessageMethod {
    fn default() -> Self {
        PinMessageMethod {
            settings: AuthenticationType::None,
            message_id: String::default(),
            server_url: String::new(),
            pin_message: true,
        }
    }
}

#[async_trait]
impl APIMethod for PinMessageMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: if self.pin_message {
                RestApiUrlType::ChatPinMessage
            } else {
                RestApiUrlType::ChatUnPinMessage
            },
            ..Default::default()
        }
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert(
            "messageId".to_string(),
            PayloadValue::String(&self.message_id),
        );
        Some(payload)
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, PinMessageMethod};
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }
    #[test]
    fn test_get_commands_values() {
        let result = PinMessageMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
            message_id: "messageId".to_string(),
            pin_message: false,
        };
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        // TODO fixme
        assert!(result.json_payload().is_none());
    }
}
