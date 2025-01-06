/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
//! Class for BannerInfo
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(default)]
pub struct TextArguments {
    text_arguments: Option<Vec<String>>,
}

#[derive(Clone, Default, Deserialize, Debug)]
#[serde(default)]
pub struct BannerInfo {
    // Text from Banner
    pub text: String,
    // Banner title
    pub title: String,
    // Url link
    pub link: String,
    // Identifier
    #[serde(rename = "id")]
    pub identifier: String,
    // Is read.
    pub read: bool,
    pub priority: i32,
    #[serde(default)]
    #[serde(rename = "textArguments")]
    pub text_arguments: TextArguments,
}

impl BannerInfo {
    pub fn new() -> Self {
        BannerInfo {
            text: String::default(),
            title: String::default(),
            link: String::default(),
            identifier: String::default(),
            read: false,
            priority: -1,
            text_arguments: TextArguments::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.text.is_empty() && !self.identifier.is_empty() && !self.title.is_empty()
    }
}

/*
    Debug output for BannerInfo
    TODO verify it
    Perhaps use debug directly.
*/
impl fmt::Display for BannerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"BannerInfo text: {}, title: {}, description {}, identifier {}, read {})"#,
            self.text, self.title, self.link, self.identifier, self.read
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bannerinfo::BannerInfo;
    use std::fs::File;
    pub fn parse(filename: &str) -> BannerInfo {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_invalid_by_default() {
        let b = BannerInfo::new();
        assert!(!b.is_valid());
        assert!(b.text.is_empty());
        assert!(b.title.is_empty());
        assert!(b.link.is_empty());
        assert!(b.identifier.is_empty());
        assert!(!b.read);
        assert_eq!(b.priority, -1);
    }

    #[test]
    fn test_parse_data() {
        {
            // Load file
            let result = parse("src/data/bannerinfo/bannerinfo1.json");
            assert!(result.is_valid());
            assert_eq!(result.text, "For all installations using SAML Please upgrade as soon as possible.  3.9.1 / 3.8.3 / 3.7.3 / 2.4.13 / 1.3.4 / 0.74.4");
            assert_eq!(result.title, "Attn: Important Security fix");
            assert_eq!(
                result.link,
                "https://github.com/RocketChat/Rocket.Chat/releases/tag/3.9.1"
            );
            assert_eq!(result.identifier, "alert-5fcc1f02f5204d09050943d2");
            assert_eq!(result.priority, 10);
            assert!(result.read);
        }
        {
            // Load file
            let result = parse("src/data/bannerinfo/bannerinfo2.json");
            assert!(result.is_valid());

            assert_eq!(result.text, "New_version_available_(s)");
            assert_eq!(result.title, "Update_your_RocketChat");
            assert_eq!(
                result.link,
                "https://github.com/RocketChat/Rocket.Chat/releases/tag/4.4.1"
            );
            assert_eq!(result.identifier, "versionUpdate-4_4_1");
            assert_eq!(result.priority, 10);
            assert!(!result.read);
        }
        {
            // Load file
            let result = parse("src/data/bannerinfo/bannerinfoempty.json");
            assert!(!result.is_valid());
            assert!(result.text.is_empty());
            assert!(result.title.is_empty());
            assert!(result.link.is_empty());
            assert!(result.identifier.is_empty());
            assert!(!result.read);
            assert_eq!(result.text_arguments.text_arguments, None);
        }
    }
}
