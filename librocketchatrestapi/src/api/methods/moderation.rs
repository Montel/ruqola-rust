/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use reqwest::Method;
use std::collections::HashMap;

use crate::api::methods::{base::PayloadValue, APIMethod};
use libauthenticationbase::authenticationsettings::AuthenticationType;
/// Implement GetModerationReportInfo
pub struct GetModerationReportInfo {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub report_id: String,
}

impl Default for GetModerationReportInfo {
    fn default() -> Self {
        GetModerationReportInfo {
            settings: AuthenticationType::None,
            report_id: String::default(),
            server_url: String::default(),
        }
    }
}

impl APIMethod for GetModerationReportInfo {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/moderation.reportInfo"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("reportId".to_string(), self.report_id.clone());
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

/// Implement GetModerationReports
pub struct GetModerationReports {
    pub settings: AuthenticationType,
    pub server_url: String,
    pub message_id: String,
}

impl Default for GetModerationReports {
    fn default() -> Self {
        GetModerationReports {
            settings: AuthenticationType::None,
            server_url: String::default(),
            message_id: String::default(),
        }
    }
}

impl APIMethod for GetModerationReports {
    fn settings(&self) -> &AuthenticationType {
        &self.settings
    }

    fn endpoint(&self) -> &str {
        "/api/v1/moderation.reports"
    }

    fn required_authentication(&self) -> bool {
        true
    }

    fn query_parameters(&self) -> Option<HashMap<String, String>> {
        let mut payload: HashMap<String, String> = HashMap::new();
        payload.insert("msgId".to_string(), self.message_id.clone());
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
    use crate::methods::{APIMethod, GetModerationReportInfo, GetModerationReports};
    use libauthenticationbase::authenticationsettings::{AuthenticationType, LoginSettings};
    use reqwest::Method;

    pub fn generate_default_settings() -> AuthenticationType {
        AuthenticationType::Login(LoginSettings {
            username: "chuck_norris".to_string(),
            password: "supersecret".to_string(),
        })
    }
    #[test]
    fn test_get_moderation_report_info_values() {
        let result = GetModerationReportInfo {
            settings: generate_default_settings(),
            report_id: "foo".to_string(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/moderation.reportInfo");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(query) = &result.query_parameters() {
            let _report_id = "foo".to_string();
            assert_eq!(query.get("reportId"), Some(&_report_id));
        } else {
            panic!("Impossble to get parameters");
        }
        assert!(result.json_payload().is_none());
    }

    #[test]
    fn test_get_moderation_reports_values() {
        let result = GetModerationReports {
            settings: generate_default_settings(),
            message_id: "foo".to_string(),
            server_url: "https://mydomain.com".to_string(),
        };
        assert_eq!(result.endpoint(), "/api/v1/moderation.reports");
        assert_eq!(result.method(), Method::GET);
        assert!(result.required_authentication());

        // Test Json values.
        if let Some(query) = &result.query_parameters() {
            let _message_id = "foo".to_string();
            assert_eq!(query.get("msgId"), Some(&_message_id));
        } else {
            panic!("Impossble to get parameters");
        }
        assert!(result.json_payload().is_none());
    }
}
