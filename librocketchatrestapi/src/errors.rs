/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[derive(Debug)]
pub enum Error {
    // trunk-ignore(clippy/enum_variant_names)
    JsonDecode(String),
    MissingSettings,
    RequestFailed(String),
    // trunk-ignore(clippy/enum_variant_names)
    ResponseText,
}
