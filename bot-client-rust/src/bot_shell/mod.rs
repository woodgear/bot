use crate::util::one_shot;
use bot::{ast::*, protocol::*};

pub mod bot_shell_ext_service;

pub struct BotShell {
    pub server: String,
}

impl BotShell {
    pub fn new(server: String) -> Self {
        Self { server }
    }

    pub async fn spawn(&self, cmd: &str) -> Result<u32, failure::Error> {
        let res = one_shot(&self.server, ServerAst::Spawn(cmd.to_string())).await?;
        let out = res.into_spawn_result();
        if out.status == 0 {
            //TODO we should return pid
            return Ok(0);
        }
        return Err(failure::err_msg(out.err_msg));
    }
    pub async fn exec(&self, cmd: &str) -> Result<String, failure::Error> {
        let res = one_shot(&self.server, ServerAst::Call(cmd.to_string())).await?;
        let out = res.into_call_result();
        if out.status == 0 {
            return Ok(out.output);
        }
        return Err(failure::err_msg(out.output));
    }
}
