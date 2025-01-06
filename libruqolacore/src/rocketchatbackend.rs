/*
 * SPDX-FileCopyrightText: 2023-2025 Laurent Montel <laurent.montel@kdab.com>
 *
 * SPDX-License-Identifier: LGPL-2.0-or-later
 */

use libauthenticationbase::authenticationsettings::AuthenticationType;
use libddpapi::ddpclient;
use libddpapi::ddpclient::Event;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct RocketaccountBackend {
    pub ddpclient_builder: ddpclient::DDpClientBuilder,
}
impl RocketaccountBackend {
    pub fn new() -> Self {
        RocketaccountBackend {
            ddpclient_builder: ddpclient::DDpClientBuilder::new(),
        }
    }
    pub fn set_settings(&mut self, setting: AuthenticationType) {
        self.ddpclient_builder.set_settings(setting);
    }

    pub async fn build(
        self,
    ) -> Result<(ddpclient::DDpClient, mpsc::UnboundedReceiver<Event>), ddpclient::BuilderError>
    {
        self.ddpclient_builder.build().await
    }
}

impl Default for RocketaccountBackend {
    fn default() -> Self {
        RocketaccountBackend::new()
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
