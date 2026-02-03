/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[derive(Debug, Clone)]
pub struct LoginSettings {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AuthSettings {
    pub auth_token: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
pub enum AuthenticationType {
    None,
    Login(LoginSettings),
    Auth(AuthSettings),
    NoAuthRequired,
}
