/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use std::collections::HashMap;

use crate::api::methods::base::EndPointInfo;
use crate::api::methods::base::PayloadValue;
use crate::api::methods::restapiutils::RestApiUrlType;
use crate::api::methods::APIMethod;
use libauthenticationbase::authenticationsettings::AuthenticationType;
use reqwest::Method;
pub struct ChannelCreateMethod {
    pub settings: AuthenticationType,
    pub server_url: String,

    pub name: String,
    pub members: Option<Vec<String>>,
    pub read_only: Option<bool>,
}

impl Default for ChannelCreateMethod {
    fn default() -> Self {
        ChannelCreateMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
            name: String::default(),
            members: None,
            read_only: Some(false),
        }
    }
}

impl APIMethod for ChannelCreateMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChannelsCreate,
            ..Default::default()
        }
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("name".to_string(), PayloadValue::String(&self.name));

        if let Some(members) = &self.members {
            let mem = members.iter().map(|el| el.as_ref()).collect();
            payload.insert("members".to_string(), PayloadValue::ListOfString(mem));
        }
        if let Some(read_only) = &self.read_only {
            payload.insert("readOnly".to_string(), PayloadValue::Bool(read_only));
        }

        Some(payload)
    }
}

// ChannelRemoveModeratorJob
pub struct ChannelRemoveModeratorJob {
    pub settings: AuthenticationType,

    pub server_url: String,
    pub user_id: String,
    pub channel_id: String,
}

impl Default for ChannelRemoveModeratorJob {
    fn default() -> Self {
        ChannelRemoveModeratorJob {
            settings: AuthenticationType::None,
            user_id: String::default(),
            channel_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for ChannelRemoveModeratorJob {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChannelsRemoveModerator,
            ..Default::default()
        }
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("userId".to_string(), PayloadValue::String(&self.user_id));
        payload.insert("roomId".to_string(), PayloadValue::String(&self.channel_id));

        Some(payload)
    }
}

//ChannelRemoveLeaderJob
pub struct ChannelRemoveLeaderMethod {
    pub settings: AuthenticationType,

    pub server_url: String,
    pub user_id: String,
    pub channel_id: String,
}

impl Default for ChannelRemoveLeaderMethod {
    fn default() -> Self {
        ChannelRemoveLeaderMethod {
            settings: AuthenticationType::None,
            user_id: String::default(),
            channel_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for ChannelRemoveLeaderMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn domain(&self) -> &str {
        &self.server_url
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::ChannelsRemoveLeader,
            ..Default::default()
        }
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("userId".to_string(), PayloadValue::String(&self.user_id));
        payload.insert("roomId".to_string(), PayloadValue::String(&self.channel_id));

        Some(payload)
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::{
        APIMethod, ChannelCreateMethod, ChannelRemoveLeaderMethod, ChannelRemoveModeratorJob,
        PayloadValue,
    };
    use assert_matches::assert_matches;
    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
    use reqwest::Method;

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }

    #[test]
    fn test_channels_create_values() {
        let result = ChannelCreateMethod {
            settings: generate_default_settings(),
            name: "some-channel".to_string(),
            members: Some(vec!["rocket.cat".to_string()]),
            ..Default::default()
        };
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(json) = &result.json_payload() {
            assert_matches!(json.get("name"), Some(PayloadValue::String("some-channel")));
            assert_matches!(json.get("name"), Some(PayloadValue::String("some-channel")));
            assert_matches!(json.get("readOnly"), Some(PayloadValue::Bool(false)));
            let _list_members = ["rocket.cat"];
            assert_matches!(
                json.get("members"),
                Some(PayloadValue::ListOfString(_list_members))
            );
        } else {
            panic!("Impossble to get parameters");
        }
    }

    #[test]
    fn test_channels_remove_leader_values() {
        let result = ChannelRemoveLeaderMethod {
            settings: generate_default_settings(),
            user_id: "user_id1".to_string(),
            channel_id: "channel id1".to_string(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(json) = &result.json_payload() {
            assert_matches!(
                json.get("userId"),
                Some(PayloadValue::String(r#"user_id1"#))
            );
            assert_matches!(
                json.get("roomId"),
                Some(PayloadValue::String("channel id1"))
            );
        } else {
            panic!("Impossble to get parameters");
        }
    }

    #[test]
    fn test_channels_remove_moderator_values() {
        let result = ChannelRemoveModeratorJob {
            settings: generate_default_settings(),
            user_id: "user_id1".to_string(),
            channel_id: "channel id1".to_string(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(json) = &result.json_payload() {
            assert_matches!(json.get("userId"), Some(PayloadValue::String("user_id1")));
            assert_matches!(
                json.get("roomId"),
                Some(PayloadValue::String("channel id1"))
            );
        } else {
            panic!("Impossble to get parameters");
        }
    }
}
