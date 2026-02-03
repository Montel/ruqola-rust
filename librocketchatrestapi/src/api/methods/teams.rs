/*
* SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/

use reqwest::Method;
use std::collections::HashMap;

use crate::api::methods::base::EndPointInfo;
use crate::api::methods::base::PayloadValue;
use crate::api::methods::restapiutils::RestApiUrlType;
use crate::api::methods::APIMethod;
use libauthenticationbase::authenticationsettings::AuthenticationType;

/// Implement teams.delete
pub struct GetTeamsListMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    // TODO add pattern
}

impl Default for GetTeamsListMethod {
    fn default() -> Self {
        GetTeamsListMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetTeamsListMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::TeamsList,
            ..Default::default()
        }
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

/// Implement GetTeamInfoMethod
pub struct GetTeamInfoMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub team_id: String,
}

impl Default for GetTeamInfoMethod {
    fn default() -> Self {
        GetTeamInfoMethod {
            settings: AuthenticationType::None,
            team_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetTeamInfoMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::TeamsInfo,
            ..Default::default()
        }
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("teamId".to_string(), self.team_id.clone());
        Some(payload)
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

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, GetTeamInfoMethod, GetTeamsListMethod};
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }
    #[test]
    fn test_get_team_info_values() {
        let result = GetTeamInfoMethod {
            settings: generate_default_settings(),
            server_url: "https://www.kde.org".to_string(),
            team_id: "foo".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(query) = &result.query_parameters() {
            let _team_id: String = "foo".to_string();
            assert_eq!(query.get("teamId"), Some(&_team_id));
        } else {
            panic!("Impossble to get parameters");
        }
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_get_teams_list_values() {
        let loginsettings = generate_default_settings();
        let result = GetTeamsListMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
}
