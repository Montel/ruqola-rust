/*
 * SPDX-FileCopyrightText: 2024-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::file::File;
use serde::Deserialize;

#[derive(Clone, Deserialize, Default, Debug)]
#[serde(default)]
pub struct FileAttachments {
    pub files: Vec<File>,
    pub count: u64,
    pub offset: u64,
    pub total: u64,
}

impl FileAttachments {
    pub fn new() -> Self {
        FileAttachments {
            files: Vec::<File>::default(),
            ..Default::default()
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<FileAttachments>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fileattachments::FileAttachments;
    use std::fs::File;
    // For test !
    pub fn parse(filename: &str) -> FileAttachments {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }
    #[test]
    fn test_is_empty() {
        let b = FileAttachments::new();
        assert!(b.files.is_empty());
        assert_eq!(b.count, 0);
        assert_eq!(b.offset, 0);
        assert_eq!(b.total, 0);
    }

    #[test]
    fn test_parse_data() {
        {
            let b = parse("src/data/fileattachments/fileattachments1.json");
            assert!(!b.files.is_empty());
            assert_eq!(b.count, 20);
            assert_eq!(b.offset, 0);
            assert_eq!(b.total, 20);
        }
        {
            let b = parse("src/data/fileattachments/fileattachments2.json");
            assert!(!b.files.is_empty());
            assert_eq!(b.count, 1);
            assert_eq!(b.offset, 0);
            assert_eq!(b.total, 1);
        }
        {
            let b = parse("src/data/fileattachments/fileattachments3.json");
            assert!(b.files.is_empty());
            assert_eq!(b.count, 0);
            assert_eq!(b.offset, 0);
            assert_eq!(b.total, 0);
        }
    }
}
