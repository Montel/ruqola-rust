/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::ddpmessage::NoSubInfo;
use crate::registration::{registration_list, registration_room_list};
use crate::{authenticationutils, ddpmessage, ddputils, rocketchatmessage};
use futures::stream::{SplitSink, SplitStream};
use futures::SinkExt;
use futures_util::StreamExt;
use libauthenticationbase::authenticationsettings::AuthenticationType;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

// https://stackoverflow.com/questions/77277773/multiple-owners-to-a-tokio-tungstenite-wss-stream

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypingInfo {
    room_id: String,
    user_id: String,
    typing: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Command {
    Disconnect,
    SendMessage(String, String), /* message, room_id */
    ChangeDefaultStatus(String),
    ChangeTypingStatus(TypingInfo),
    SubscribeRoom(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Event {
    MessageReceived(String),
    ErrorReceived(String),
    ChangeElementType(ddpmessage::ChangeElementType),
    AddElementType(ddpmessage::AddElementType),
    RemoveElementType(ddpmessage::RemoveElementType),
    ResultReceived(serde_json::Value),
}

struct DDpClientTask {
    from_ws: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    to_ws: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    command_receiver: mpsc::UnboundedReceiver<Command>,
    event_sender: mpsc::UnboundedSender<Event>,
    method_identifier: u64,
    user_name: String,
}

pub struct DDpClient {
    task: tokio::task::JoinHandle<()>,
    command_sender: mpsc::UnboundedSender<Command>,
}

pub enum BuilderError {
    MissingUrl,
    InvalidAccountSettings,
}
#[derive(Clone)]
pub struct DDpClientBuilder {
    websocket_url: Option<String>,
    settings: AuthenticationType,
}

impl Default for DDpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DDpClientBuilder {
    pub fn new() -> Self {
        DDpClientBuilder {
            websocket_url: None,
            settings: AuthenticationType::None,
        }
    }
    pub fn set_websocket_url(&mut self, url: String) {
        self.websocket_url = Some(url);
    }

    pub fn set_settings(&mut self, settings: AuthenticationType) {
        self.settings = settings;
    }
    pub fn settings(&self) -> AuthenticationType {
        self.settings.clone()
    }

    pub async fn build(self) -> Result<(DDpClient, mpsc::UnboundedReceiver<Event>), BuilderError> {
        let Some(url) = self.websocket_url else {
            return Err(BuilderError::MissingUrl);
        };

        // Adapt url
        let modified_url = ddputils::generate_websocket_url(url.to_string());
        // println!("modified url {}", modified_url);
        let (ws_stream, _) = connect_async(modified_url).await.unwrap();
        println!(r#"WebSocket handshake has been successfully completed"#);

        // Split from/to websocket
        let (mut to_ws, from_ws) = ws_stream.split();

        let connect_str = authenticationutils::send_connect().to_string();
        let message = Message::Text(connect_str.to_string());
        to_ws
            .send(message)
            .await
            .expect("Impossible to send connect message");

        let mut user_name = String::new();
        match &self.settings {
            AuthenticationType::None => {
                println!("Not settings")
            }
            AuthenticationType::NoAuthRequired => {
                println!("No Auth Required")
            }
            // TOOD we need to use identifier
            AuthenticationType::Login(login_settings) => {
                // TODO fix identifier value !
                let login_str = authenticationutils::generate_method(
                    String::from("login"),
                    authenticationutils::login_ldap(
                        &login_settings.username,
                        &login_settings.password,
                    ),
                    // self.method_identifier
                    1,
                );

                let message = Message::Text(login_str.to_string());
                println!("{:?}", login_str);
                to_ws.send(message).await.expect("Impossible to login");
                println!("loggin settings");
                user_name = login_settings.username.clone();
            }
            AuthenticationType::Auth(auth_settings) => {
                let resume_str = authenticationutils::generate_method(
                    String::from("login"),
                    authenticationutils::login_resume(&auth_settings.auth_token),
                    // self.method_identifier,
                    1,
                );

                let message = Message::Text(resume_str.to_string());
                println!("{:?}", resume_str);
                to_ws.send(message).await.expect("Impossible to resume");
                println!("AuthSettings settings");
            }
        }

        // create command channel
        let (command_sender, command_receiver) = mpsc::unbounded_channel();

        //Create event Channel
        let (event_sender, event_receiver): (mpsc::UnboundedSender<_>, mpsc::UnboundedReceiver<_>) =
            mpsc::unbounded_channel();

        // Create task
        let task_data = DDpClientTask {
            from_ws,
            to_ws,
            command_receiver,
            event_sender,
            method_identifier: 1,
            user_name,
        };

        // Add task
        let task = tokio::spawn(async move {
            task_data.run().await;
        });

        let client = DDpClient {
            task,
            command_sender,
        };

        Ok((client, event_receiver))
    }
}

impl DDpClientTask {
    async fn run(mut self) {
        loop {
            tokio::select! {
            // Test websocket message
            Some(ws_message) = self.from_ws.next() => {
                println!("Message reçu : {:?}", ws_message);
                let parse_message = ddpmessage::parse_received_message(ws_message.unwrap());
                match parse_message {
                    ddpmessage::MessageReceivedType::PingMessage => {
                        self.send_pong().await;
                    },
                    ddpmessage::MessageReceivedType::ConnectedMessage => {
                        println!("SS connectedmessage");
                        //self.send_connected().await;
                    },
                    ddpmessage::MessageReceivedType::ElementAdded(added_element) => {
                        println!("SS added_elements");
                        match added_element.clone() {
                            ddpmessage::AddElementType::User(user_added) => {
                                if user_added.user_name == self.user_name {
                                    println!("current user !!!!! {:?}", user_added);
                                    // self.account_settings.user_name;
                                    self.send_registration(user_added.user_id).await;
                                }
                            },
                            _ => println!("OTHER"),
                        };
                        self.added_elements(added_element).await;
                    },
                    ddpmessage::MessageReceivedType::ElementChanged(changed_element) => {
                        self.changed_elements(changed_element).await;
                    },
                    ddpmessage::MessageReceivedType::ElementRemoved(remove_element) => {
                        self.remove_elements(remove_element).await;
                    },
                    ddpmessage::MessageReceivedType::NoSub(text) =>  {
                        self.no_subscribe(text)
                    },
                    ddpmessage::MessageReceivedType::Ready(string) => {
                        println!("READY***********************");
                        // TODO need to change status ???
                    },
                    ddpmessage::MessageReceivedType::Error(str) => {
                        self.error_message(str)
                    },
                    ddpmessage::MessageReceivedType::Unknown(text) => {
                        self.unknown_message(text)
                    },
                    ddpmessage::MessageReceivedType::Result(text) => {
                        self.result_message(text)
                    },
                    ddpmessage::MessageReceivedType::InvalidText => {
                        self.invalid_text();
                    }
                }
            }
            ,
            // Test command message
            Some(command) = self.command_receiver.recv() => {
                use Command::*;
                match command {
                    Disconnect => {
                        self.disconnect().await;
                        // We kill task.
                    }
                    SendMessage(message, room_id) => {
                        self.send_message(message, room_id).await
                    }
                    ChangeDefaultStatus(status) => {
                        self.send_default_status(status).await
                    }
                    ChangeTypingStatus(info) => {
                        self.send_change_typing_info(info).await
                    }
                    SubscribeRoom(room_id) => {
                        self.subscribe_room(room_id).await
                    }
                    }
                }
            }
        }
    }

    async fn disconnect(&mut self) {
        println!("Disconnected");
        // TODO
    }

    async fn send_message(&mut self, message: String, room_id: String) {
        // TODO generate message => send to ws
        // Necessary ???? => use restapi for it.
    }

    async fn send_change_typing_info(&mut self, info: TypingInfo) {
        println!("Change typing status {:?}", info);
        let change_typing_str = rocketchatmessage::generate_inform_typing_status(
            info.room_id,
            info.user_id,
            info.typing,
            &mut self.method_identifier,
        );
        self.to_ws
            .send(Message::Text(change_typing_str))
            .await
            .expect("Impossible change typing info");
    }

    async fn send_default_status(&mut self, status: String) {
        println!("Change default status {}", status);
        let change_status_str = rocketchatmessage::generate_method_set_default_status(
            status,
            &mut self.method_identifier,
        );
        self.to_ws
            .send(Message::Text(change_status_str))
            .await
            .expect("Impossible change status");
    }

    async fn added_elements(&mut self, elements: ddpmessage::AddElementType) {
        println!("add elements {:?}", elements);
        self.event_sender
            .send(Event::AddElementType(elements))
            .expect("Impossible to send add elements type message");
    }

    async fn changed_elements(&mut self, elements: ddpmessage::ChangeElementType) {
        println!("change element {:?}", elements);
        self.event_sender
            .send(Event::ChangeElementType(elements))
            .expect("Impossible to send change element type message");
    }

    async fn remove_elements(&mut self, elements: ddpmessage::RemoveElementType) {
        println!("remove elements {:?}", elements);
        self.event_sender
            .send(Event::RemoveElementType(elements))
            .expect("Impossible to send remove element type message");
    }

    async fn send_pong(&mut self) {
        let send_pong_str = authenticationutils::send_pong().to_string();
        println!("SEND PONG");
        self.to_ws
            .send(Message::Text(send_pong_str))
            .await
            .expect("Impossible to send pong");
    }

    // Send connected
    pub async fn send_connected(&mut self) {
        let connect_str = authenticationutils::send_connect().to_string();
        self.to_ws
            .send(Message::Text(connect_str))
            .await
            .expect("Impossible to send connect");
    }

    pub async fn send_registration(&mut self, user_id: String) {
        let subscriptionvalue = rocketchatmessage::subscription_get(&mut self.method_identifier);
        self.to_ws
            .send(Message::Text(subscriptionvalue))
            .await
            .expect("Impossible to send subscription get");

        let result = registration_list(user_id, &mut self.method_identifier);
        for r in result.iter() {
            println!("registration {:?}", r);
            self.to_ws
                .send(Message::Text(r.to_string()))
                .await
                .expect("Impossible to send registration");
        }

        let roomsvalue = rocketchatmessage::rooms_get(&mut self.method_identifier);
        self.to_ws
            .send(Message::Text(roomsvalue))
            .await
            .expect("Impossible to send rooms get");
    }

    fn error_message(&self, str: String) {
        println!("error message {:?}", str);
        self.event_sender
            .send(Event::ErrorReceived(str))
            .expect("Impossible to send error message");
    }

    fn unknown_message(&self, str: String) {
        println!("Unknown message {:?}", str);
        self.event_sender
            .send(Event::ErrorReceived(str))
            .expect("Impossible to send error message");
    }

    fn result_message(&self, val: serde_json::Value) {
        println!("result message {:?}", val);
        self.event_sender
            .send(Event::ResultReceived(val))
            .expect("Impossible to send result message");
    }

    fn invalid_text(&self) {
        println!("invalid text");
    }

    fn no_subscribe(&self, info: NoSubInfo) {
        println!("No subscribe: {}", info);
    }
    async fn subscribe_room(&mut self, room_id: String) {
        println!("Subscribe room {}", room_id);
        let result = registration_room_list(room_id, &mut self.method_identifier);
        for r in result.iter() {
            self.to_ws
                .send(Message::Text(r.to_string()))
                .await
                .expect("Impossible to send subscription room");
        }
    }
}

impl DDpClient {
    pub async fn disconnect(self) {
        self.command_sender.send(Command::Disconnect).unwrap();
        self.task.await.unwrap();
    }

    pub fn send_message(&self, message: String, room_id: String) {
        self.command_sender
            .send(Command::SendMessage(message, room_id))
            .unwrap();
    }

    pub fn change_default_status(&self, status: String) {
        self.command_sender
            .send(Command::ChangeDefaultStatus(status))
            .unwrap();
    }

    pub fn change_typing_info(&self, info: TypingInfo) {
        self.command_sender
            .send(Command::ChangeTypingStatus(info))
            .unwrap();
    }

    pub fn subscribe_room(&self, room_id: String) {
        self.command_sender
            .send(Command::SubscribeRoom(room_id))
            .unwrap();
    }
}
