/*
* SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
struct BlockAccessoryOption {
    text: String,
    value: String,
}

impl BlockAccessoryOption {
    pub fn new() -> Self {
        BlockAccessoryOption {
            text: String::new(),
            value: String::new(),
        }
    }
}

/*
Debug output for MessageAttachmentField
*/
impl fmt::Display for BlockAccessoryOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockAccessoryOption(text: {}, value {})",
            self.text, self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::blockaccessoryoption::BlockAccessoryOption;

    #[test]
    fn test_blockaccessoryoption_default() {
        let block_accessory_option = BlockAccessoryOption::new();
        assert!(block_accessory_option.value.is_empty());
        assert!(block_accessory_option.text.is_empty());
    }
}
