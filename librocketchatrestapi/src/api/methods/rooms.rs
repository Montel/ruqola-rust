/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use std::collections::HashMap;

use reqwest::Method;

use crate::api::methods::base::PayloadValue;
use crate::api::methods::APIMethod;
use libauthenticationbase::authenticationsettings::AuthenticationType;
/// Implement GetRooms
pub struct GetRoomsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for GetRoomsMethod {
    fn default() -> Self {
        GetRoomsMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetRoomsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/rooms.get"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }
}

/// Implement GetDiscussions
pub struct GetDiscussionsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub room_id: String,
}

impl Default for GetDiscussionsMethod {
    fn default() -> Self {
        GetDiscussionsMethod {
            settings: AuthenticationType::None,
            room_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetDiscussionsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/rooms.getDiscussions"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("roomId".to_string(), self.room_id.clone());
        Some(payload)
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }
}

/// Implement GetRoomInfo
pub struct GetRoomInfoMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub room_id: String,
}

impl Default for GetRoomInfoMethod {
    fn default() -> Self {
        GetRoomInfoMethod {
            settings: AuthenticationType::None,
            room_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetRoomInfoMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/rooms.info"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("roomId".to_string(), self.room_id.clone());
        Some(payload)
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        None
    }
    fn domain(&self) -> &str {
        &self.server_url
    }
}

/// Implement ChangeRoomFavoriteMethod
pub struct ChangeRoomFavoriteMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub room_id: String,
    pub favorite: bool,
}

impl Default for ChangeRoomFavoriteMethod {
    fn default() -> Self {
        ChangeRoomFavoriteMethod {
            settings: AuthenticationType::None,
            room_id: String::default(),
            favorite: true,
            server_url: String::default(),
        }
    }
}

impl APIMethod for ChangeRoomFavoriteMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/rooms.favorite"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("favorite".to_string(), PayloadValue::Bool(&self.favorite));
        payload.insert("roomId".to_string(), PayloadValue::String(&self.room_id));

        Some(payload)
    }
    fn domain(&self) -> &str {
        &self.server_url
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::APIMethod;
    use crate::methods::{
        ChangeRoomFavoriteMethod, GetDiscussionsMethod, GetRoomsMethod, PayloadValue,
    };
    use assert_matches::assert_matches;
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }

    #[test]
    fn test_get_rooms_values() {
        let result = GetRoomsMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/rooms.get");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_get_discussions_values() {
        let result = GetDiscussionsMethod {
            settings: generate_default_settings(),
            room_id: "foo".to_string(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/rooms.getDiscussions");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        // Test Json values.
        if let Some(query) = &result.query_parameters() {
            let _team_id = "foo".to_string();
            assert_matches!(query.get("roomId"), _team_id);
        } else {
            panic!("Impossble to get parameters");
        }
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_rooms_change_favorite_true() {
        let result = ChangeRoomFavoriteMethod {
            settings: generate_default_settings(),
            room_id: "room_id1".to_string(),
            favorite: true,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/rooms.favorite");
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(json) = &result.json_payload() {
            assert_matches!(
                json.get("roomId"),
                Some(PayloadValue::String(r#"room_id1"#))
            );
            assert_matches!(json.get("favorite"), Some(PayloadValue::Bool(true)));
        } else {
            panic!("Impossble to get parameters");
        }
    }
    #[test]
    fn test_rooms_change_favorite_false() {
        let result = ChangeRoomFavoriteMethod {
            settings: generate_default_settings(),
            room_id: "room_id2".to_string(),
            favorite: false,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/rooms.favorite");
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(json) = &result.json_payload() {
            assert_matches!(
                json.get("roomId"),
                Some(PayloadValue::String(r#"room_id2"#))
            );
            assert_matches!(json.get("favorite"), Some(PayloadValue::Bool(false)));
        } else {
            panic!("Impossble to get parameters");
        }
    }
}
