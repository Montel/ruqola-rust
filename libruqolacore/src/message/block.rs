/*
 * SPDX-FileCopyrightText: 2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
enum BlockType {
    Unknown,
    VideoConf,
    Actions,
    Section,
}
#[derive(Clone, Default, Deserialize, Debug)]
pub struct Block {}
