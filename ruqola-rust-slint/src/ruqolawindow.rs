/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use libruqolacore::{
    rocketchataccountmanager::{CommandFromGui, RocketChatAccountManager},
    rocketchataccountsettings::RocketChatAccountSettings,
};

use tokio::sync::mpsc;

pub mod ui {
    slint::include_modules!();
}

use slint::*;
use std::rc::Rc;
use ui::*;
pub struct Gui {}

impl Gui {
    pub async fn ui_loop(user_inputs_out: mpsc::UnboundedSender<CommandFromGui>) {
        let main_window = RuqolaWindow::new().unwrap();

        let user_inputs_out_clone = user_inputs_out.clone();
        let user_inputs_out_clone_connect = user_inputs_out.clone();
        let user_inputs_out_clone_disconnect = user_inputs_out.clone();

        main_window.on_quit(move || {
            slint::quit_event_loop().unwrap();
        });

        main_window.on_disconnect_account(move || {
            // TODO change it
            let account_name = String::from("");
            user_inputs_out_clone_disconnect
                .send(CommandFromGui::DisconnectAccount { account_name })
                .unwrap();
            println!("disconnect connect account");
        });
        main_window.on_connect_account(move |index| {
            // TODO change it
            let account_name = String::from("");
            user_inputs_out_clone_connect
                .send(CommandFromGui::ConnectAccount { account_name })
                .unwrap();
            println!("connect account {index}");
        });

        let weak_window = main_window.as_weak();
        main_window.on_show_accounts(move || {
            let account_model = VecModel::default();
            let mut manager = RocketChatAccountManager::new();
            manager.load_accounts();
            let accounts = manager.list_accounts();
            for acc in &accounts {
                account_model.push(StandardListViewItem::from(slint::format!("{}", acc)));
                println!("{acc}");
            }
            let main_window = weak_window.upgrade().unwrap();
            main_window.set_account_model(Rc::new(account_model).into());
        });

        main_window.on_send_message(move |str| {
            user_inputs_out
                .send(CommandFromGui::SendMessage {
                    message: str.to_string(),
                    room_id: "".to_string(),
                })
                .unwrap();

            println!("send message {:?}", str);
        });

        main_window.on_show_configure_accounts(move || {
            let account_settings_dialog: AccountSettingsDialog =
                AccountSettingsDialog::new().unwrap();
            let account_model = VecModel::default();

            let mut manager = RocketChatAccountManager::new();
            manager.load_accounts();
            let accounts = manager.list_accounts();
            for acc in &accounts {
                account_model.push(StandardListViewItem::from(slint::format!("{}", acc)));
            }

            account_settings_dialog.set_account_model(Rc::new(account_model).into());

            let dialogweak = account_settings_dialog.as_weak();
            dialogweak
                .unwrap()
                .show()
                .expect("Impossible to create dialog");

            dialogweak.unwrap().on_close_clicked(move || {
                println!("close clicked");
                dialogweak
                    .unwrap()
                    .hide()
                    .expect("Impossible to hide dialog");
            });

            let user_inputs_out_clone = user_inputs_out_clone.clone();
            let user_inputs_out_clone_remove = user_inputs_out_clone.clone();
            let dialogweak = account_settings_dialog.as_weak();
            dialogweak.unwrap().on_add_new_account_clicked(move || {
                let dialog: AccountConfigurationDialog = AccountConfigurationDialog::new().unwrap();
                let dialogweak = dialog.as_weak();
                dialogweak
                    .unwrap()
                    .show()
                    .expect("Impossible to create dialog");

                let user_inputs_out_clone = user_inputs_out_clone.clone();
                dialogweak
                    .unwrap()
                    .on_create_update_account(move |account_info| {
                        let account_info = account_info.clone();
                        let mut account_settings: RocketChatAccountSettings =
                            RocketChatAccountSettings::new();
                        account_settings.account_name =
                            String::from(account_info.account_name.as_str());
                        account_settings.server_url_name =
                            String::from(account_info.server_url.as_str());
                        account_settings.user_name = String::from(account_info.user_name.as_str());
                        account_settings.password = String::from(account_info.password.as_str());

                        // Send channel info
                        user_inputs_out_clone
                            .send(CommandFromGui::AddAccount { account_settings })
                            .unwrap();

                        println!("add account {:?}", account_info);
                        dialogweak
                            .unwrap()
                            .hide()
                            .expect("Impossible to hide dialog");
                    });

                let dialogweak = dialog.as_weak();
                dialogweak.unwrap().on_cancel_button_clicked(move || {
                    println!("close clicked");
                    dialogweak
                        .unwrap()
                        .hide()
                        .expect("Impossible to hide dialog");
                });
                println!("add new account");
            });

            dialogweak.unwrap().on_modify_account_clicked(move || {
                let dialog: AccountConfigurationDialog = AccountConfigurationDialog::new().unwrap();
                dialog.show().expect("Impossible to create dialog");

                println!("modify new account");
            });

            dialogweak.unwrap().on_remove_account_clicked(move |index| {
                println!("remove account");
                user_inputs_out_clone_remove
                    .send(CommandFromGui::RemoveAccount { index_name: index })
                    .unwrap();
            });
        });

        tokio::task::block_in_place(|| main_window.run().unwrap());
    }
}
