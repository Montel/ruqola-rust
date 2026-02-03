/*
 * SPDX-FileCopyrightText: 2025-2026 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

// TODO need to implement bitwise
/*
pub enum PasswordSettingCheck {
   none(0), // We need to have 0!
   minLengh(1 << 0),
   maxLengh(1 << 1),
   forbidRepeatingCharactersCount(1 << 2),
   forbidRepeatingCharacters(1 << 4),
   atLeastOneLowercase(1 << 8),
   atLeastOneUppercase(1 << 16),
   atLeastOneSpecialCharacter(1 << 32),
   atLeastOneNumber(1 << 64);

   final int value;
   const PasswordSettingCheck(this.value);
 }
 */
#[derive(Clone, Debug, PartialEq)]
pub struct RuqolaServerConfigPassword {
    pub accounts_password_policy_min_length: i64,
    pub accounts_password_policy_max_length: i64,
    pub accounts_password_policy_forbid_repeating_characters_count: i64,
    pub accounts_password_policy_enabled: bool,
    pub accounts_password_policy_forbid_repeating_characters: bool,
    pub accounts_password_policy_at_least_one_lowercase: bool,
    pub accounts_password_policy_at_least_one_uppercase: bool,
    pub accounts_password_policy_at_least_one_number: bool,
    pub accounts_password_policy_at_least_one_special_character: bool,
}

impl Default for RuqolaServerConfigPassword {
    fn default() -> Self {
        RuqolaServerConfigPassword::new()
    }
}

impl RuqolaServerConfigPassword {
    pub fn new() -> Self {
        RuqolaServerConfigPassword {
            accounts_password_policy_min_length: 12,
            accounts_password_policy_max_length: 24,
            accounts_password_policy_forbid_repeating_characters_count: 3,
            accounts_password_policy_enabled: false,
            accounts_password_policy_forbid_repeating_characters: true,
            accounts_password_policy_at_least_one_lowercase: true,
            accounts_password_policy_at_least_one_uppercase: true,
            accounts_password_policy_at_least_one_number: true,
            accounts_password_policy_at_least_one_special_character: true,
        }
    }
}
