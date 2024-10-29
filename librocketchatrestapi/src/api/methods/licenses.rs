/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use reqwest::Method;
use std::collections::HashMap;

use crate::api::methods::{base::PayloadValue, APIMethod};
use libauthenticationbase::authenticationsettings::AuthenticationType;

/// Implement LicensesIsEnterprise
pub struct LicensesIsEnterpriseMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for LicensesIsEnterpriseMethod {
    fn default() -> Self {
        LicensesIsEnterpriseMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for LicensesIsEnterpriseMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/licenses.isEnterprise"
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

// LicensesListMethod
pub struct LicensesListMethod {
    pub settings: AuthenticationType,
    pub server_url: String,
}

impl Default for LicensesListMethod {
    fn default() -> Self {
        LicensesListMethod {
            settings: AuthenticationType::None,
            server_url: String::default(),
        }
    }
}

impl APIMethod for LicensesListMethod {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/licenses.get"
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

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, LicensesIsEnterpriseMethod, LicensesListMethod};
    use reqwest::Method;

    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }
    #[test]
    fn test_get_licenses_is_enterprise_values() {
        let loginsettings = generate_default_settings();
        let result = LicensesIsEnterpriseMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/licenses.isEnterprise");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_get_licenses_list_licenses_values() {
        let loginsettings = generate_default_settings();
        let result = LicensesListMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/licenses.get");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
}
