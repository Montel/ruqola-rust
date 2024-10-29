/*
 * SPDX-FileCopyrightText: 2023-2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */
#[cfg(test)]
extern crate libruqolacore;

use libruqolacore::deviceinfo;
use libruqolacore::deviceinfo::Device;
use std::fs;

fn load_file(_file_name: String) -> String {
    fs::read_to_string(_file_name).expect("Unable to read file")
}

#[test]
fn test_deviceinfo() {
    let device_info = deviceinfo::DeviceInfo::new();
    assert!(device_info.identifier.is_empty());
}

#[test]
fn test_parsing() {
    {
        // Load file
        let data = load_file(String::from("./tests/data/deviceinfo-empty.json"));
        // Parse invitation
        let device_info = deviceinfo::DeviceInfo::parse(&data);
        let expected = deviceinfo::DeviceInfo::new();

        assert_eq!(device_info, expected);
    }
    {
        // Load file
        let data = load_file(String::from("./tests/data/deviceinfo1.json"));
        // Parse invitation
        let device_info = deviceinfo::DeviceInfo::parse(&data);
        let mut expected = deviceinfo::DeviceInfo::new();
        expected.identifier = String::from("iy78NiKzd");
        expected.host = String::from("chat.kde.org");
        expected.session_id = String::from("iKzd");
        expected.ip = String::from("179.4.8.1");
        expected.user_id = String::from("acidH");
        expected.device = None::<Device>;

        assert_eq!(device_info, expected);
    }
    {
        // Load file
        let data = load_file(String::from("./tests/data/deviceinfo2.json"));
        // Parse invitation
        let device_info = deviceinfo::DeviceInfo::parse(&data);
        let mut expected = deviceinfo::DeviceInfo::new();
        expected.identifier = String::from("87pMW");
        expected.host = String::from("chat.kde.org");
        expected.session_id = String::from("87pMW");
        expected.ip = String::from("1.7.6.11");
        expected.user_id = String::from("dacidH");
        let mut os = deviceinfo::Os::new();
        os.version = String::from("x86_64");
        os.name = String::from("Linux");
        let mut device = deviceinfo::Device::new();
        device.name = String::from("Firefox");
        device.os = os;

        expected.device = Some(device);

        assert_eq!(device_info, expected);
    }
}
