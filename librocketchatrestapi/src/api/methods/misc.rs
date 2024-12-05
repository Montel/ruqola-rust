/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::api::methods::base::EndPointInfo;
use crate::api::methods::restapiutils::RestApiUrlType;
use crate::api::methods::{base::PayloadValue, APIMethod};
use libauthenticationbase::authenticationsettings::AuthenticationType;
use reqwest::Method;
use std::collections::HashMap;

// Statistic method
pub struct StatisticsMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub refresh: bool,
}

impl Default for StatisticsMethod {
    fn default() -> Self {
        StatisticsMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
            refresh: false,
        }
    }
}

impl APIMethod for StatisticsMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        if self.refresh {
            payload.insert("refresh".to_string(), "true".to_string());
        } else {
            payload.insert("refresh".to_string(), "false".to_string());
        }
        Some(payload)
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::Statistics,
            ..Default::default()
        }
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

// Own info
pub struct OwnMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for OwnMethod {
    fn default() -> Self {
        OwnMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for OwnMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::Me,
            ..Default::default()
        }
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

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, OwnMethod, StatisticsMethod};
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }

    #[test]
    fn test_get_owninfo_values() {
        let loginsettings = generate_default_settings();
        let result = OwnMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_get_statistics_values() {
        let loginsettings = generate_default_settings();
        let mut result = StatisticsMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
            refresh: false,
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_some());
        assert!(result.json_payload().is_none());

        // Test Json values.
        let _result = "false".to_string();
        if let Some(query) = &result.query_parameters() {
            assert_eq!(query.get("refresh"), Some(&_result));
        } else {
            panic!("Impossble to get parameters {:?}", _result);
        }
        result.refresh = true;
        let _result = "true".to_string();
        if let Some(query) = &result.query_parameters() {
            assert_eq!(query.get("refresh"), Some(&_result));
        } else {
            panic!("Impossble to get parameters {:?}", _result);
        }
    }
}
