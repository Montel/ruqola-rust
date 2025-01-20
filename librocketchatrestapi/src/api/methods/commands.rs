/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::api::methods::base::EndPointInfo;
use crate::api::methods::base::PayloadValue;
use crate::api::methods::restapiutils::RestApiUrlType;
use crate::api::methods::APIMethod;
use libauthenticationbase::authenticationsettings::AuthenticationType;
use reqwest::Method;
use std::collections::HashMap;

/// Implement GetCommands
pub struct GetCommandsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for GetCommandsMethod {
    fn default() -> Self {
        GetCommandsMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetCommandsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::CommandsGet,
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

/// Implement GetListCommandsMethod
pub struct GetListCommandsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for GetListCommandsMethod {
    fn default() -> Self {
        GetListCommandsMethod {
            settings: AuthenticationType::None,
            server_url: String::new(),
        }
    }
}

impl APIMethod for GetListCommandsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::CommandsList,
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

// RunCommand
pub struct RunCommandsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    // FIXME add parameters
}

impl Default for RunCommandsMethod {
    fn default() -> Self {
        RunCommandsMethod {
            settings: AuthenticationType::None,
            server_url: String::new(),
        }
    }
}

impl APIMethod for RunCommandsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::CommandsRun,
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
        Method::POST
    }

    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>> {
        // TODO
        None
    }

    fn domain(&self) -> &str {
        &self.server_url
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, GetCommandsMethod, GetListCommandsMethod, RunCommandsMethod};
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
        let result = GetCommandsMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_commands_list_values() {
        let result = GetListCommandsMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_run_commands_values() {
        let result = RunCommandsMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::POST);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
}
