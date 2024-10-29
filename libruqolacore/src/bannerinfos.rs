/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::bannerinfo::BannerInfo;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug)]
pub struct BannerInfos {
    pub banners: Vec<BannerInfo>,
}

impl Default for BannerInfos {
    fn default() -> Self {
        BannerInfos::new()
    }
}

impl BannerInfos {
    pub fn new() -> Self {
        BannerInfos {
            banners: Vec::<BannerInfo>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<BannerInfos>(json) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bannerinfos::BannerInfos;
    use std::fs::File;
    // For test !
    pub fn parse(filename: &str) -> BannerInfos {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }
    #[test]
    fn test_is_empty() {
        let b = BannerInfos::new();
        assert!(b.banners.is_empty());
    }
    #[test]
    fn test_parse_data() {
        {
            let result = parse("src/data/bannerinfos/bannerinfos1.json");
            assert_eq!(result.banners.len(), 3);
        }
        {
            let result = parse("src/data/bannerinfos/bannerinfos2.json");
            assert_eq!(result.banners.len(), 3);
        }
        {
            let result = parse("src/data/bannerinfos/bannerinfos3.json");
            assert_eq!(result.banners.len(), 3);
        }
    }
}
