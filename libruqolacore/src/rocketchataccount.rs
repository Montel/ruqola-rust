use crate::rocketchataccountmanager::CommandToBackend;
/*
* SPDX-FileCopyrightText: 2023-2026 Laurent Montel <laurent.montel@kdab.com>
*
* SPDX-License-Identifier: LGPL-2.0-or-later
*/
use crate::rooms::Rooms;
use crate::{
    rocketchataccountsettings::RocketChatAccountSettings, rocketchatbackend::RocketaccountBackend,
};
use libddpapi::ddpclient;
use libddpapi::ddpclient::Event;
use librocketchatrestapi::methods;
use tokio::sync::mpsc;
#[derive(Clone)]
pub struct RocketChatAccount {
    pub account_settings: RocketChatAccountSettings,
    pub account_backend: RocketaccountBackend,
    pub rooms: Rooms,
    // pub ddpclient: libddpapi::ddpclient,
    // Store rooms + messages
}
impl Default for RocketChatAccount {
    fn default() -> Self {
        RocketChatAccount::new()
    }
}
impl RocketChatAccount {
    pub fn new() -> Self {
        Self {
            account_settings: RocketChatAccountSettings::new(),
            account_backend: RocketaccountBackend::new(),
            rooms: Rooms::new(),
            //ddpclient: libddpapi::ddpclient::
        }
    }
    pub fn load_settings(&mut self, file_name: String) {
        self.account_settings.load_settings(file_name)
    }
    pub fn write_settings(&mut self, file_name: String) {
        self.account_settings.write_settings(file_name)
    }

    async fn send_message(&mut self, message: String) {
        // TODO generate message => send to ws
        // Necessary ???? => use restapi for it.
    }

    fn parse_result(&mut self, value: serde_json::Value) {
        self.parse_rooms(value);
    }

    fn parse_rooms(&mut self, value: serde_json::Value) {
        // println!("PARSE ROOM {:?}", value);
        let result_value = value["result"].as_object();
        if result_value.is_some() {
            let update_value = result_value.unwrap();
            if update_value.contains_key("update") {
                let update_value: Option<&Vec<serde_json::Value>> =
                    update_value["update"].as_array();
                if let Some(update_values) = update_value {
                    println!("Has update rooms");
                    self.rooms.parse_insert_rooms(update_values);
                    println!("NOMBER OF ROOMS {:?}", self.rooms.rooms.len());
                    println!("ROOMS************************* {:?}", self.rooms.rooms);
                }
            } else {
                println!("Not a room!!!!! ");
            }
        }
    }

    fn parse_element_changed(&mut self, change_type: libddpapi::ddpmessage::ChangeElementType) {
        match change_type {
            libddpapi::ddpmessage::ChangeElementType::User => println!("USER"),
            libddpapi::ddpmessage::ChangeElementType::Rooms => println!("ROOM"),
            libddpapi::ddpmessage::ChangeElementType::StreamRoomMessage(value) => {
                println!("ROOM MESSAGE")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserOtr(value) => {
                println!("Notification OTR")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserUiInteraction(value) => {
                println!("UiInteraction {}", value)
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserNotification(value) => {
                println!("NOTIFICATION")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserWebrtc(value) => {
                println!("WebRTC")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserMessage(value) => {
                println!("USER MESSAGE")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserUserData(value) => {
                println!("USER DATA")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserRoomsChanged(value) => {
                println!("ROOMS CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserVideoConference(value) => {
                println!("VIDEO CONFERENCE ")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserSubscriptionsChanged(
                value,
            ) => println!("USER SUBSCRIPTION CHANGED"),
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserForceLogout(value) => {
                println!("FORCE LOGOUT")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyUserUnknown(value) => {
                println!("USER UNKNOWN")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyRoomDeleteMessage => {
                println!("DELETE MESSAGE")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyRoomUserActivity => {
                println!("USER ACTIVITY")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyLogged => println!("LOGGED"),
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyRoomUnknown => {
                println!("ROOM UNKNOWN")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyAllUnknown => println!("UNKNONW"),
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyAllDeleteCustomSound(value) => {
                println!("DELETE CUSTOM SOUND")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamNotifyAllUpdateCustomSound(value) => {
                println!("UPDATE CUSTOM SOUND")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamStout(value) => {
                println!("STREAM STDOUT")
            }
            libddpapi::ddpmessage::ChangeElementType::RolesChange(value) => {
                println!("ROLES CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::UpdateAvatar(value) => {
                println!("UPDATE AVATAR")
            }
            libddpapi::ddpmessage::ChangeElementType::UpdateEmojiCustom(value) => {
                println!("UPDATE EMOJI CUSTOM")
            }
            libddpapi::ddpmessage::ChangeElementType::UsersNameChanged(value) => {
                println!("USERNAME CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::UserDeleted(value) => {
                println!("DELETE USER")
            }
            libddpapi::ddpmessage::ChangeElementType::DeleteCustomUserStatus(value) => {
                println!("DELETE CUSTOM USER STATUS")
            }
            libddpapi::ddpmessage::ChangeElementType::UpdateCustomUserStatus(value) => {
                println!("UPDATE CUSTOM USER STATUS")
            }
            libddpapi::ddpmessage::ChangeElementType::UserStatus(svalue) => {
                println!("USER STATUS CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::DeleteEmojiCustom(value) => {
                println!("DELETE EMOJI CUSTOM")
            }
            libddpapi::ddpmessage::ChangeElementType::PermissionsChanged(value) => {
                println!("PERMISSION CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::PrivateSettingsChanged(value) => {
                println!("PRIVATE SETTINGS CHANGED")
            }
            libddpapi::ddpmessage::ChangeElementType::StreamRoles(value) => {
                println!("STREAM ROLE")
            }
            libddpapi::ddpmessage::ChangeElementType::Unknown => println!("PROBLEM UNKNOWN"),
        }
    }

    pub async fn build(
        mut self,
        mut receiver: mpsc::UnboundedReceiver<CommandToBackend>,
        sender: mpsc::UnboundedSender<CommandToBackend>,
    ) -> Result<(ddpclient::DDpClient, mpsc::UnboundedReceiver<Event>), ddpclient::BuilderError>
    {
        // Verify if we can connect or not
        match self.account_backend.ddpclient_builder.clone().build().await {
            Err(b_error) => match b_error {
                ddpclient::BuilderError::MissingUrl => {
                    // TODO log! perhaps sezrching rust module for it.
                    println!("Error url");
                    Err(b_error)
                }
                ddpclient::BuilderError::InvalidAccountSettings => {
                    println!("Invalid Account Settings");
                    Err(b_error)
                } // TODO add more error
            },
            Ok((ddpclient, mut event_receiver)) => {
                println!("CONNECTED ");
                loop {
                    tokio::select! {
                        Some(receiver) = receiver.recv() => {
                            match receiver {
                                CommandToBackend::SendMessage {message, room_id} => {
                                    ddpclient.send_message(message, room_id);
                                }
                                _ => println!("Receive a element")
                            }
                        }
                        Some(event) = event_receiver.recv() => {
                            match event {
                                ddpclient::Event::MessageReceived(str) => {
                                    println!("Message received {:?}", str);
                                },
                                ddpclient::Event::ErrorReceived(str) => {
                                    println!("Error received {:?}", str);
                                },
                                ddpclient::Event::ResultReceived(str) => {
                                    //println!("Result received {:?}", str);
                                    self.parse_result(str);
                                },
                                ddpclient::Event::ChangeElementType(element_type) => {
                                    println!("Change Elements received {:?}", element_type);
                                    self.parse_element_changed(element_type)
                                },
                                ddpclient::Event::AddElementType(element_type) => {
                                    println!("Add Elements received {:?}", element_type);
                                    match element_type {
                                        libddpapi::ddpmessage::AddElementType::User(user_added) => {
                                            // TODO check username!
                                            if user_added.user_name == self.account_settings.user_name {
                                                println!("current user !!!!!");
                                                // _ddpclient.command_sender
                                                // register user
                                            }
                                            println!("USER ADDED")
                                        },
                                        _ => println!("OTHER"),
                                    };
                                },
                                ddpclient::Event::RemoveElementType(element_type) => {
                                    println!("Remove Elements received {:?}", element_type);
                                },
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn initialize_accounts(&self) {
        let (sender, receiver) = mpsc::unbounded_channel::<CommandToBackend>();
        <RocketChatAccount as Clone>::clone(&self)
            .build(receiver, sender)
            .await;
    }
}
