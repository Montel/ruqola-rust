/*
 * SPDX-FileCopyrightText: 2024 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

pub fn generate_websocket_url(url: String) -> String {
    let mut new_url = url;
    if new_url.is_empty() {
        return String::from("");
    }
    if new_url.starts_with("https://") {
        new_url = new_url.replace("https://", "wss://");
    } else if new_url.starts_with("http://") {
        new_url = new_url.replace("http://", "ws://");
    } else if !new_url.starts_with("wss://") {
        new_url = String::from("wss://") + &new_url;
    }
    if !new_url.ends_with("/websocket") {
        new_url = new_url + &String::from("/websocket");
    }
    new_url
}

#[cfg(test)]
mod tests {
    use crate::ddputils;
    #[test]
    fn test_generate_url() {
        let mut url = String::from("");
        assert_eq!(ddputils::generate_websocket_url(url), "");
        url = String::from("http://foo.kde.org/");
        assert_eq!(
            ddputils::generate_websocket_url(url),
            "ws://foo.kde.org//websocket"
        );
        url = String::from("http://foo.kde.org");
        assert_eq!(
            ddputils::generate_websocket_url(url),
            "ws://foo.kde.org/websocket"
        );
        url = String::from("foo.kde.org");
        assert_eq!(
            ddputils::generate_websocket_url(url),
            "wss://foo.kde.org/websocket"
        );
        url = String::from("foo.kde.org/websocket");
        assert_eq!(
            ddputils::generate_websocket_url(url),
            "wss://foo.kde.org/websocket"
        );
        url = String::from("wss://foo.kde.org/websocket");
        assert_eq!(
            ddputils::generate_websocket_url(url),
            "wss://foo.kde.org/websocket"
        );
    }
}
