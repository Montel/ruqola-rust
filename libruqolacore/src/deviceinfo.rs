/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct DeviceInfo {
    #[serde(rename = "_id")]
    pub identifier: String,
    pub host: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub ip: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub device: Option<Device>,
    // pub mLoginAt : i64,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            identifier: String::default(),
            host: String::default(),
            session_id: String::default(),
            ip: String::default(),
            user_id: String::default(),
            device: None::<Device>,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Os {
    pub version: String,
    pub name: String,
}

impl Os {
    pub fn new() -> Os {
        Os {
            version: String::default(),
            name: String::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Device {
    pub name: String,
    pub os: Os,
}

impl Device {
    pub fn new() -> Device {
        Device {
            name: String::default(),
            os: Os::new(),
        }
    }
}

impl DeviceInfo {
    pub fn parse(filename: &str) -> DeviceInfo {
        serde_json::from_str(filename).expect("JSON was not well-formatted")
    }
}

// Debug output for Os
// TODO verify it
impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name {}, version {})", self.name, self.version)
    }
}
