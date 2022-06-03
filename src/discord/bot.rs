pub struct BotClient {
    config: BotConfig
}

pub struct BotConfig {
    token: String,
    pub login_msg: Option<String>
}

impl BotConfig {
    pub fn new(token: String) -> Self {
        Self {
            token,
            login_msg: None
        }
    }

    pub fn set_login_msg(&mut self, login_msg: String) {
        self.login_msg = Some(login_msg);
    }
}