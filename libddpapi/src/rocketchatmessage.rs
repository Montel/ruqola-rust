use std::ops::Add;

/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
use serde_json::json;

use crate::authenticationutils;
// Load all rooms
pub fn rooms_get(identifier: &mut u64) -> String {
    *identifier = identifier.add(1);
    let value = json!([{"$date":0}]);
    authenticationutils::generate_method(String::from("rooms/get"), value, *identifier)
}

pub fn subscription_get(identifier: &mut u64) -> String {
    *identifier = identifier.add(1);
    let value = json!([{"$date":0}]);
    //{\"id\":\"3\",\"method\":\"subscriptions/get\",\"msg\":\"method\",\"params\":[{\"$date\":0}]}
    authenticationutils::generate_method(String::from("subscriptions/get"), value, *identifier)
}

pub fn generate_method_set_default_status(status_str: String, identifier: &mut u64) -> String {
    let value = json!([status_str]);
    *identifier = identifier.add(1);
    authenticationutils::generate_method(
        String::from("UserPresence:setDefaultStatus"),
        value,
        *identifier,
    )
}

pub fn generate_inform_typing_status(
    room_id: String,
    user_id: String,
    typing_status: bool,
    identifier: &mut u64,
) -> String {
    let event_name = room_id + "/user-activity";
    *identifier = identifier.add(1);
    let value = json!([event_name, user_id, typing_status]);

    authenticationutils::generate_method(String::from("stream-notify-room"), value, *identifier)
}

pub fn subscribe(name: String, params: String, identifier: u64) -> String {
    let value = json!(
        {
            "msg": "sub",
            "id": identifier.to_string(),
            "name": name,
            "params": [
                params,
                {
                    "useCollection": false,
                    "args": []
                }
            ]
        }
    );
    value.to_string()
}

#[cfg(test)]
mod tests {
    use crate::rocketchatmessage;
    #[test]
    fn test_generate_method_set_default_status() {
        let status_str = String::from("away");
        let mut value = 5;
        assert_eq!(
            rocketchatmessage::generate_method_set_default_status(status_str, &mut value),
            r#"{"id":"5","method":"UserPresence:setDefaultStatus","msg":"method","params":["away"]}"#
        );
    }
    #[test]
    fn test_generate_inform_typing_status() {
        let mut value = 5;
        assert_eq!(
            rocketchatmessage::generate_inform_typing_status(
                String::from("room42"),
                String::from("user42"),
                true,
                &mut value
            ),
            r#"{"id":"5","method":"stream-notify-room","msg":"method","params":["room42/user-activity","user42",true]}"#
        );
    }

    #[test]
    fn test_subscribe() {
        let value = String::from("room_id/deleteMessage");
        assert_eq!(
            rocketchatmessage::subscribe(String::from("method_name"), value, 5),
            r#"{"id":"5","msg":"sub","name":"method_name","params":["room_id/deleteMessage",{"args":[],"useCollection":false}]}"#
        );
    }

    #[test]
    fn test_subscription_get() {
        let mut value = 5_u64;
        let result = rocketchatmessage::subscription_get(&mut value);
        assert_eq!(
            result,
            r#"{"id":"6","method":"subscriptions/get","msg":"method","params":[{"$date":0}]}"#
        );
    }

    #[test]
    fn test_rooms_get() {
        let mut value = 5_u64;
        let result = rocketchatmessage::rooms_get(&mut value);
        assert_eq!(
            result,
            r#"{"id":"6","method":"rooms/get","msg":"method","params":[{"$date":0}]}"#
        );
    }
}
