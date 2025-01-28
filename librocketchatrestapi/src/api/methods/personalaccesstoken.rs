/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
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

// TODO
