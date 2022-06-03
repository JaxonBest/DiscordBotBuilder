// JS DISCORD CLIENT

use crate::discord::bot::BotClient;

pub struct JsClient {
    client: BotClient
}

impl JsClient {
    pub fn new(client: BotClient) -> Self {
        Self { client }
    }

    pub fn build_imports() -> String {
        "".to_string()
    }
}