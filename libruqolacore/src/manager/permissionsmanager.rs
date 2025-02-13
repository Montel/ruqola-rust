/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

#[derive(Clone, Debug, PartialEq)]
pub struct PermissionsManager {}

#[cfg(test)]
mod tests {
    use crate::manager::permissionsmanager::PermissionsManager;
    // TODO
    /*
    #[test]
    fn test_default_values() {
        let r = LicensesManager::new();
        assert!(r.licenses.is_empty());
    }

    #[test]
    fn test_contains_license() {
        let mut r = LicensesManager::new();
        r.licenses.push("dd".to_string());
        r.licenses.push("license2".to_string());

        assert!(!r.licenses.is_empty());
        assert!(r.licenses.len() == 2);
        assert!(r.has_license(String::from("dd")));
        assert!(!r.has_license(String::from("dd2")));
    }
    */
}
