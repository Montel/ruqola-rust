/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use std::collections::HashMap;

use crate::api::methods::base::EndPointInfo;
use crate::api::methods::restapiutils::RestApiUrlType;
use reqwest::Method;

use crate::api::methods::base::PayloadValue;
use crate::api::methods::APIMethod;
use libauthenticationbase::authenticationsettings::AuthenticationType;
pub struct PermissionsListAllMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for PermissionsListAllMethod {
    fn default() -> Self {
        PermissionsListAllMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for PermissionsListAllMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        None
    }

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::PermissionsListAll,
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
    use crate::methods::{APIMethod, PermissionsListAllMethod};
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }
    #[test]
    fn test_get_permissions_values() {
        let result = PermissionsListAllMethod {
            settings: generate_default_settings(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
}
