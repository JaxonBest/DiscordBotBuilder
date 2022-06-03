// JS DISCORD CLIENT

use crate::discord::bot::BotClient;

use super::{js_import::JsImport, js_types::JsString};

pub struct JsClient {
    client: BotClient
}

impl JsClient {
    pub fn new(client: BotClient) -> Self {
        Self { client }
    }

    pub fn build_imports(extras: Option<Vec<JsImport>>) -> String {
        let discordjs = JsImport::from_string_imports(
            vec!["Client".to_string(), "Intents".to_string()],
            JsString::from_str("discord.js"));
        let fs_import = JsImport::new_default("fs".to_string(), JsString::from_str("fs"));
        let mut extra_imports = String::new();

        match extras {
            Some(r) => {
                for im in r {
                    extra_imports.push_str(&format!("{}\n", im.build_import()));
                }
            },
            _ => {}
        }

        format!("{};{};\n{}", discordjs.build_import(), fs_import.build_import(), extra_imports)
    }
}