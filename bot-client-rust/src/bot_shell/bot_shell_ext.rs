use crate::bot_shell::BotShell;
use bot::{ast::*, protocol::*};

impl BotShell {
    pub async fn service_exist(&self, name: &str) -> Result<bool, failure::Error> {
        let res = self
            .exec(ExecConfig {
                cmd: format!("sc query {}", name),
                cwd: "".to_string(),
            })
            .await;
        if let Err(e) = res {
            if e.to_string().contains("1060") {
                return Ok(false);
            } else {
                return Err(e);
            }
        }
        return Ok(true);
    }

    pub async fn service_running(&self, name: &str) -> Result<bool, failure::Error> {
        let res = self
            .exec(ExecConfig {
                cmd: format!("sc query {}", name),
                cwd: "".to_string(),
            })
            .await?;
        return Ok(res.to_ascii_lowercase().contains("running"));
    }
}


impl BotShell {
    pub async fn process_exist(&self, name: &str) -> Result<bool, failure::Error> { 
        
    }
}