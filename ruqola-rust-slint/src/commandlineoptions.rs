/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    /// Optional => we can use without command
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = false)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Account {
        /// The account name
        accountname: String,
    },
    #[command(arg_required_else_help = false)]
    ListAccount {},
    #[command(arg_required_else_help = false)]
    Console {},
}
