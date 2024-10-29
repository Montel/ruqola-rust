/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use crate::room::Room;
use serde::Deserialize;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Rooms {
    pub rooms: Vec<Room>,
}

impl Default for Rooms {
    fn default() -> Self {
        Rooms::new()
    }
}

impl Rooms {
    pub fn new() -> Self {
        Rooms {
            rooms: Vec::<Room>::default(),
        }
    }
    pub fn parse_elements(&mut self, json: &str) {
        if let Ok(val) = serde_json::from_str::<Rooms>(json) {
            *self = val
        }
    }

    pub fn parse_insert_rooms(&mut self, insertroom: &Vec<Value>) {
        insertroom.iter().for_each(|room: &Value| {
            if let Some(j) = room.as_object() {
                let room_id;
                if j.contains_key("rid") {
                    room_id = j["rid"].as_str().unwrap().to_string();
                } else {
                    room_id = j["_id"].as_str().unwrap().to_string();
                }

                for r in self.rooms.iter() {
                    println!(" room in list {:?} search {:?}", r.room_id, room_id);
                    if r.room_id == room_id {
                        println!(
                            "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG existing room {:?}",
                            room_id
                        );
                        break;
                    }
                }
            }

            let mut r = Room::new();
            r.parse_insert_room(room);
            if r.is_valid() {
                self.rooms.push(r);
            }
        });
    }

    pub fn parse_update_element(&mut self, json: &Value) {
        if let Ok(val) = serde_json::from_value::<Rooms>(json.clone()) {
            *self = val
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::rooms::Rooms;

    // For test !
    pub fn parse(filename: &str) -> Rooms {
        let file = File::open(filename).expect("Failed to open file");
        serde_json::from_reader(file).expect("JSON was not well-formatted")
    }

    #[test]
    fn test_is_empty() {
        let b = Rooms::new();
        assert!(b.rooms.is_empty());
    }
}
