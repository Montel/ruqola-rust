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

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::LicensesIsEntreprise,
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

    fn endpointinfo(&self) -> EndPointInfo {
        EndPointInfo {
            endpoint_type: RestApiUrlType::LicensesGet,
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

#[cfg(test)]
mod tests {
    use crate::methods::{APIMethod, LicensesIsEnterpriseMethod, LicensesListMethod};
    use reqwest::Method;
    use crate::api::methods::restapiutils::RestApiUrlType;

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
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
        assert_eq!(result.endpointinfo().endpoint_type, RestApiUrlType::LicensesIsEntreprise);
    }

    #[test]
    fn test_get_licenses_list_licenses_values() {
        let loginsettings = generate_default_settings();
        let result = LicensesListMethod {
            settings: loginsettings,
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());
        assert!(result.query_parameters().is_none());
        assert!(result.json_payload().is_none());
    }
}
