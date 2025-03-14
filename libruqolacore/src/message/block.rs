/*
* SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/
use serde::Deserialize;
use std::fmt;

use super::blockaction::BlockAction;

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
pub struct BlockActions {
    pub blockactions: Vec<BlockAction>,
}

impl BlockActions {
    pub fn new() -> Self {
        BlockActions {
            blockactions: Vec::new(),
        }
    }
}

#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub enum BlockType {
    #[default]
    Unknown,
    #[serde(alias = "video_conf")]
    VideoConf,
    #[serde(alias = "actions")]
    Actions,
    #[serde(alias = "section")]
    Section,
    #[serde(alias = "context")]
    Context,
    #[serde(alias = "divider")]
    Divider,
    #[serde(alias = "overflow")]
    Overflow,
    #[serde(alias = "image")]
    Image,
    #[serde(alias = "preview")]
    Preview,
    #[serde(alias = "callout")]
    Callout,
}

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Block {
    #[serde(rename = "blockId")]
    pub block_id: String,
    #[serde(rename = "callId")]
    pub call_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    pub section_text: String,
    #[serde(rename = "type")]
    pub block_type: BlockType,
    #[serde(rename = "elements")]
    pub block_actions: BlockActions,
}

impl Block {
    pub fn new() -> Self {
        Block {
            block_id: String::new(),
            call_id: String::new(),
            app_id: String::new(),
            section_text: String::new(),
            block_type: BlockType::Unknown,
            block_actions: BlockActions::new(),
        }
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block(blockId: {},callId: {},appId {},sectionText {}, blockType {:?}, block_action {:?} )",
            self.block_id, self.call_id, self.app_id, self.section_text, self.block_type, self.block_actions
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::block::{Block, BlockType};

    #[test]
    fn test_block_default() {
        let block = Block::new();
        assert!(block.block_id.is_empty());
        assert!(block.call_id.is_empty());
        assert!(block.app_id.is_empty());
        assert!(block.section_text.is_empty());
        assert_eq!(block.block_type, BlockType::Unknown);
        assert!(block.block_actions.blockactions.is_empty());
    }
}
