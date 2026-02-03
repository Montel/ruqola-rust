/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use crate::user::Status;
use std::fmt;
#[derive(Clone, Debug)]
pub struct DisplayStatusInfo {
    pub display_text: String,
    pub status_str: String,
    pub icon_name: String,
    pub order: i64,
    pub status: Status,
}

impl Default for DisplayStatusInfo {
    fn default() -> Self {
        DisplayStatusInfo::new()
    }
}

impl DisplayStatusInfo {
    fn new() -> Self {
        DisplayStatusInfo {
            display_text: String::default(),
            status_str: String::default(),
            icon_name: String::default(),
            order: -1,
            status: Status::Unknown,
        }
    }
}

/*
  Debug output for DisplayStatusInfo
*/
impl fmt::Display for DisplayStatusInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DisplayStatusInfo(display_text: {}, status_str {}, icon_name {}, order {}, status {:?})",
            self.display_text, self.status_str, self.icon_name, self.order, self.status,
        )
    }
}

#[derive(Clone, Debug)]
pub struct StatusModel {
    pub list_status: Vec<DisplayStatusInfo>,
}

impl Default for StatusModel {
    fn default() -> Self {
        StatusModel::new()
    }
}

impl StatusModel {
    fn new() -> Self {
        StatusModel {
            list_status: Vec::default(),
        }
    }

    fn sorted_list(self) -> Vec<DisplayStatusInfo> {
        let mut sorted_list = self.list_status.clone();
        sorted_list.sort_by(|a, b| a.order.cmp(&b.order));
        sorted_list
    }
}

/*
  Debug output for DisplayStatusInfo
*/
impl fmt::Display for StatusModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusModel(list_status: {:?})", self.list_status)
    }
}

#[cfg(test)]
mod tests {
    use crate::status::statusmodel::{DisplayStatusInfo, StatusModel};
    use crate::user::Status;
    #[test]
    fn test_status_model_default() {
        let b = StatusModel::new();
        assert!(b.list_status.is_empty());
    }

    #[test]
    fn test_displaystatusinfo_default() {
        let b = DisplayStatusInfo::new();
        assert!(b.display_text.is_empty());
        assert!(b.status_str.is_empty());
        assert!(b.icon_name.is_empty());
        assert_eq!(b.order, -1);
        assert_eq!(b.status, Status::Unknown);
    }
}
