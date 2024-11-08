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

// Invite job
pub struct InviteListMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for InviteListMethod {
    fn default() -> Self {
        InviteListMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for InviteListMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpoint(&self) -> &str {
        "/api/v1/listInvites"
    }

    fn required_authentication(&self) -> bool {
        true
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

// Send invitation

pub struct SendInvitationEmailMethod<'a> {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub emails: Vec<&'a str>,
}

impl Default for SendInvitationEmailMethod<'_> {
    fn default() -> Self {
        SendInvitationEmailMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
            emails: vec![],
        }
    }
}

impl APIMethod for SendInvitationEmailMethod<'_> {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpoint(&self) -> &str {
        "/api/v1/sendInvitationEmail"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert(
            "emails".to_string(),
            PayloadValue::ListOfString(self.emails.clone()),
        );
        Some(payload)
    }

    fn domain(&self) -> &str {
        &self.server_url
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, InviteListMethod, PayloadValue, SendInvitationEmailMethod};
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
    fn test_get_invite_list_values() {
        let loginsettings = generate_default_settings();
        let result = InviteListMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/listInvites");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
    #[test]
    fn test_send_invitation_values() {
        let loginsettings = generate_default_settings();
        let result = SendInvitationEmailMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
            emails: vec!["foo@kde.org", "bla@kde.org"],
        };
        assert_eq!(result.endpoint(), "/api/v1/sendInvitationEmail");
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_some());
        // Test Json values.
        /*
        if let Some(json) = &result.json_payload() {
            assert_matches!(
                json.get("roomId"),
                Some(PayloadValue::String(r#"room_id2"#))
            );
            assert_matches!(
                json.get("emails"),
                Some(PayloadValue::ListOfString(vec<&'a str>["foo@kde.org", "bla@kde.org"]))
            );
            // TODO check json
        }
        */
    }
}
