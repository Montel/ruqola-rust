/*
* SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/
use serde::Deserialize;
use std::fmt;

#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub enum BlockType {
    #[default]
    Unknown,
    VideoConf,
    Actions,
    Section,
}
#[derive(Clone, Default, Deserialize, Debug)]
pub struct Block {
    pub block_id: String,
    pub call_id: String,
    pub app_id: String,
    pub block_str: String,
    pub section_text: String,
    pub block_type: BlockType,
}

impl Block {
    pub fn new() -> Self {
        Block {
            block_id: String::new(),
            call_id: String::new(),
            app_id: String::new(),
            block_str: String::new(),
            section_text: String::new(),
            block_type: BlockType::Unknown,
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
            "Block(blockId: {},callId: {},appId {},blockStr {},sectionText {}, blockType {:?} )",
            self.block_id,
            self.call_id,
            self.app_id,
            self.block_str,
            self.section_text,
            self.block_type
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
        assert!(block.block_str.is_empty());
        assert!(block.section_text.is_empty());
        assert_eq!(block.block_type, BlockType::Unknown);
    }
}
