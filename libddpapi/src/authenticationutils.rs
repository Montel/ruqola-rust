/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde_json::json;
use sha256::digest;

fn convert_password_ssha256(password: String) -> String {
    digest(password)
}

pub fn generate_method(method_name: String, params: serde_json::Value, identifier: u64) -> String {
    let value = json!(
        {
            "msg": "method",
            "method": method_name,
            "id": identifier.to_string(),
            "params":
                params
        }
    );
    value.to_string()
}

pub fn send_connect() -> serde_json::Value {
    let value = json!(
        {
            "msg":"connect",
            "version":"1",
            "support":["1","pre2","pre1"]
        }
    );
    value
}

pub fn send_pong() -> serde_json::Value {
    let value = json!(
        {
            "msg":"pong"
        }
    );
    value
}

pub fn login_resume(token: &str) -> serde_json::Value {
    let value = json!([
        {
            "resume": token
        }
    ]);
    value
}

pub fn login_ldap(user: &str, password: &str) -> serde_json::Value {
    let value = json!([
        {
            "ldap": true,
            "username": user,
            "ldapPass": password,
            "ldapOptions": {}
        }
    ]);
    value
}

pub fn login_oauth(credential_token: &str, credential_secret: &str) -> serde_json::Value {
    let value = json!([
        {
            "oauth":
            {
                "credentialToken": credential_token,
                "credentialSecret": credential_secret,
            }
        }
    ]);
    value
}

pub fn login(user: &str, password: &str) -> serde_json::Value {
    let password_256 = convert_password_ssha256(password.to_string());

    let user_json = match user.contains('@') {
        true => json!(
            {
                "email": user
            }
        ),
        false => json!(
            {
                "username": user
            }
        ),
    };

    let value = json!([
        {
            "password":
            {
                "algorithm": "sha-256",
                "digest": password_256,
            },
            "user": user_json
        }
    ]);
    value
}

#[cfg(test)]
mod tests {
    use crate::authenticationutils;

    #[test]
    fn test_convert_password_ssha256() {
        assert_eq!(
            authenticationutils::convert_password_ssha256(String::from("foo")),
            "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"
        );
    }

    #[test]
    fn test_login_resume() {
        assert_eq!(
            authenticationutils::login_resume("dd").to_string(),
            r#"[{"resume":"dd"}]"#
        );
    }

    #[test]
    fn test_login_ldap() {
        assert_eq!(
            authenticationutils::login_ldap("user1", "foo").to_string(),
            r#"[{"ldap":true,"ldapOptions":{},"ldapPass":"foo","username":"user1"}]"#
        );
    }

    #[test]
    fn test_login_oauth() {
        assert_eq!(
            authenticationutils::login_oauth("token", "secret").to_string(),
            r#"[{"oauth":{"credentialSecret":"secret","credentialToken":"token"}}]"#
        );
    }

    #[test]
    fn test_login() {
        assert_eq!(
            authenticationutils::login("bla", "foo").to_string(),
            r#"[{"password":{"algorithm":"sha-256","digest":"2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"},"user":{"username":"bla"}}]"#
        );
        assert_eq!(
            authenticationutils::login("bla@kde.org", "foo").to_string(),
            r#"[{"password":{"algorithm":"sha-256","digest":"2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"},"user":{"email":"bla@kde.org"}}]"#
        );
    }
}
