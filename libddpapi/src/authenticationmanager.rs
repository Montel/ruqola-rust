/*
 * SPDX-FileCopyrightText: 2024-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[derive(Debug)]
pub enum LoginStatus {
    Connecting,
    LoginOngoing,
    LoggedIn,
    LoginFailedInvalidUserOrPassword,
    LoginOtpRequired,
    LoginOtpAuthOngoing,
    LoginFailedInvalidOtp,
    LogoutOngoing,
    LoggedOut,
    LogoutCleanUpOngoing,
    LoggedOutAndCleanedUp,
    FailedToLoginPluginProblem,
    LoginFailedUserNotActivated,
    LoginFailedLoginBlockForIp,
    LoginFailedLoginBlockedForUser,
    LoginFailedLoginAppNotAllowedToLogin,
    GenericError,
}
