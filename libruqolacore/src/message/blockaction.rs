/*
* SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
pub struct ButtonAction {
    pub text: String,
}

impl ButtonAction {
    pub fn new() -> Self {
        ButtonAction {
            text: String::new(),
        }
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for ButtonAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ButtonAction(text: {})", self.text)
    }
}

#[derive(Clone, Default, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct BlockAction {
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "text")]
    pub button: ButtonAction,
    #[serde(rename = "type")]
    pub type_block: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
    pub value: String,
    pub url: String,
}

impl BlockAction {
    pub fn new() -> Self {
        BlockAction {
            action_id: String::new(),
            button: ButtonAction::new(),
            type_block: String::new(),
            block_id: String::new(),
            value: String::new(),
            url: String::new(),
        }
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for BlockAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockAction(action_id: {}, button: {}, type_block: {}, block_id: {}, url: {}, value: {})",
            self.action_id, self.button, self.type_block, self.block_id, self.url, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::blockaction::BlockAction;

    #[test]
    fn test_blockaction_default() {
        let block_action = BlockAction::new();
        assert!(block_action.action_id.is_empty());
        assert!(block_action.type_block.is_empty());
        assert!(block_action.block_id.is_empty());
        assert!(block_action.value.is_empty());
        assert!(block_action.url.is_empty());
        assert!(block_action.button.text.is_empty());
    }
}
