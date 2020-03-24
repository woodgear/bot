use crate::bot_fs::BotFs;
use crate::bot_shell::BotShell;
use crate::bot_tail::BotTail;

pub struct BotClient {
    server: String,
}

impl BotClient {
    pub fn new(url: &str) -> Self {
        BotClient {
            server: url.to_string(),
        }
    }

    /// return a instace of botfs in which you could contal fs
    pub fn fs(&self) -> BotFs {
        BotFs {
            server: self.server.to_string(),
        }
    }

    pub fn tail(&self) -> BotTail {
        unimplemented!();
    }

    pub fn shell(&self) -> BotShell {
        BotShell {
            server: self.server.to_string(),
        }
    }

    pub async fn version(&self) -> String {
        unimplemented!();
    }
}
