/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(default)]
pub struct ChannelInfo {
    pub fname: String,
    pub name: String,
    #[serde(rename = "_id")]
    pub identifier: String,
}

impl ChannelInfo {
    pub fn new() -> Self {
        ChannelInfo {
            ..Default::default()
        }
    }
}

/*
Debug output for ChannelInfo
*/
impl fmt::Display for ChannelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChannelInfo(fname: {}, name: {}, identifier {})",
            self.fname, self.name, self.identifier
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::channelinfo::ChannelInfo;

    #[test]
    fn test_channelinfo_default() {
        let channel_info = ChannelInfo::new();
        assert!(channel_info.fname.is_empty());
        assert!(channel_info.name.is_empty());
        assert!(channel_info.identifier.is_empty());
    }
}
