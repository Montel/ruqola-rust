/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use std::collections::HashMap;

use crate::errors::Error;
use async_trait::async_trait;
use libauthenticationbase::authenticationsettings::AuthenticationType;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

use super::restapiutils::{self, generate_url};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthData {
    pub user_id: String,
    pub auth_token: String,
}

#[derive(Debug, Deserialize)]
struct LoginResult {
    pub data: AuthData,
}

#[derive(Debug)]
pub struct EndPointInfo {
    pub url_extension: String,
    pub endpoint_type: restapiutils::RestApiUrlType,
    pub extension_type: restapiutils::RestApiUrlExtensionType,
}

impl Default for EndPointInfo {
    fn default() -> EndPointInfo {
        EndPointInfo {
            url_extension: String::from(""),
            endpoint_type: restapiutils::RestApiUrlType::Unknown,
            extension_type: restapiutils::RestApiUrlExtensionType::V1,
        }
    }
}

#[derive(Debug)]
pub enum PayloadValue<'a> {
    String(&'a str),
    Bool(&'a bool),
    ListOfString(Vec<&'a str>),
}

impl Serialize for PayloadValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PayloadValue::String(val) => serializer.serialize_str(val),
            PayloadValue::Bool(val) => serializer.serialize_bool(**val),
            PayloadValue::ListOfString(val) => {
                let mut seq = serializer.serialize_seq(Some(val.len()))?;
                for el in val {
                    seq.serialize_element(el)?;
                }
                seq.end()
            }
        }
    }
}

#[async_trait]
pub trait APIMethod {
    fn settings(&self) -> &AuthenticationType;
    fn method(&self) -> Method;
    fn required_authentication(&self) -> bool;
    fn json_payload(&self) -> Option<HashMap<String, PayloadValue>>;
    fn query_parameters(&self) -> Option<HashMap<String, String>>;
    fn domain(&self) -> &str;

    fn endpointinfo(&self) -> EndPointInfo;

    fn build_endpoint(&self) -> Result<String, Error> {
        if self.domain().is_empty() {
            return Err(Error::MissingSettings);
        }
        let endpointinfo = self.endpointinfo();
        let result = generate_url(
            self.domain().to_string(),
            endpointinfo.endpoint_type,
            endpointinfo.extension_type,
            endpointinfo.url_extension,
        );
        Ok(result)
    }

    // trunk-ignore(clippy/private_in_public)
    async fn request(
        &self,
        endpoint: String,
        method: Method,
        json_map: Option<&HashMap<String, PayloadValue>>,
        auth_data: Option<AuthData>,
        auth_query: Option<&HashMap<String, String>>,
    ) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();
        // TODO remove auth_data when it's not required ?
        if let Some(data) = &auth_data {
            let auth_token_hdr: &str = "x-auth-token";
            headers.insert(
                HeaderName::from_static(auth_token_hdr),
                HeaderValue::from_str(data.auth_token.as_str()).unwrap(),
            );

            let user_id_hdr: &str = "x-user-id";
            headers.insert(
                HeaderName::from_static(user_id_hdr),
                HeaderValue::from_str(data.user_id.as_str()).unwrap(),
            );
        }

        let client = Client::default();
        let mut request = client.request(method, endpoint).headers(headers);
        if let Some(json_result) = &json_map {
            request = request.json(json_result);
        }
        if let Some(query) = &auth_query {
            request = request.query(query);
        }

        match request.send().await {
            Ok(response) => Ok(response),
            Err(err) => {
                let msg = err.to_string();
                Err(Error::RequestFailed(msg))
            }
        }
    }

    fn login_payload<'a>(
        &'a self,
        username: &'a str,
        password: &'a str,
    ) -> HashMap<String, PayloadValue> {
        let mut payload: HashMap<String, PayloadValue> = HashMap::new();
        payload.insert("user".to_string(), PayloadValue::String(username));
        payload.insert("password".to_string(), PayloadValue::String(password));

        payload
    }

    // trunk-ignore(clippy/private_in_public)
    async fn login(&self, username: &str, password: &str) -> Result<AuthData, Error> {
        let response = self
            .request(
                self.build_endpoint()?,
                Method::POST,
                Some(&self.login_payload(username, password)),
                None,
                None,
            )
            .await?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = err.to_string();
            return Err(Error::RequestFailed(msg));
        }

        let result: Result<LoginResult, _> = response.json().await;
        match result {
            Ok(login_result) => Ok(login_result.data),
            Err(err) => {
                let msg = err.to_string();
                Err(Error::JsonDecode(msg))
            }
        }
    }

    async fn call(&self) -> Result<String, Error> {
        let auth_data = match self.settings() {
            AuthenticationType::None => Err(Error::MissingSettings),
            AuthenticationType::NoAuthRequired => Ok(AuthData {
                auth_token: String::default(),
                user_id: String::default(),
            }),
            AuthenticationType::Login(settings) => {
                // TODO: add processing and return LoginError
                Ok(self.login(&settings.username, &settings.password).await?)
            }
            AuthenticationType::Auth(settings) => Ok(AuthData {
                auth_token: settings.auth_token.clone(),
                user_id: settings.user_id.clone(),
            }),
        }?;

        let response = self
            .request(
                self.build_endpoint()?,
                self.method(),
                self.json_payload().as_ref(),
                Some(auth_data),
                None,
            )
            .await?;

        if let Err(err) = response.error_for_status_ref() {
            let msg = err.to_string();
            return Err(Error::RequestFailed(msg));
        }

        match response.text().await {
            Ok(text) => Ok(text),
            Err(_) => Err(Error::ResponseText),
        }
    }
}
