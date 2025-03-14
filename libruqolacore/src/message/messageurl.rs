/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;
#[derive(Default, Clone, Deserialize, Debug, PartialEq)]
pub enum ContentType {
    #[default]
    None,
    #[serde(alias = "image")]
    Image,
    #[serde(alias = "image_animated")]
    ImageAnimated,
    #[serde(alias = "audio")]
    Audio,
    #[serde(alias = "video")]
    Video,
}
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MessageUrl {
    pub url: String,
    pub page_title: String,
    pub description: String,
    pub image_url: String,
    pub author_name: String,
    pub author_url: String,
    pub site_url: String,
    pub site_name: String,
    pub url_id: String,
    pub html_description: String,
    pub image_build_url: String,
    pub image_height: i64,
    pub image_width: i64,
    pub show_preview: bool,
    pub content_type: ContentType,
}

impl Default for MessageUrl {
    fn default() -> Self {
        MessageUrl::new()
    }
}

impl MessageUrl {
    fn new() -> Self {
        MessageUrl {
            url: String::new(),
            page_title: String::new(),
            description: String::new(),
            image_url: String::new(),
            author_name: String::new(),
            author_url: String::new(),
            site_url: String::new(),
            site_name: String::new(),
            url_id: String::new(),
            html_description: String::new(),
            image_build_url: String::new(),
            image_height: -1,
            image_width: -1,
            show_preview: false,
            content_type: ContentType::None,
        }
    }
}
