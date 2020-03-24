use crate::util::one_shot;
use bot::{ast::*, protocol::*};

pub struct BotFs {
   pub server: String,
}

impl BotFs {
    pub fn new(server: String) -> Self {
        Self { server }
    }

    pub async fn read_file(&self, path: String) -> Result<Vec<u8>, failure::Error> {
        let res = one_shot(&self.server, ServerAst::ReadFile(path)).await?;
        let res = res
            .into_read_file_result()
            .0
            .map_err(|e| failure::err_msg(e))?;
        Ok(res)
    }

    pub async fn create_dir_all(&self, path: String) -> Result<(), failure::Error> {
        unimplemented!();
    }

    pub async fn write_file(&self, data: Vec<u8>) -> Result<(), failure::Error> {
        unimplemented!();
    }

    pub async fn meta(&self) {
        unimplemented!();
    }

    pub async fn copy_dir(&self, local: &str, remote: &str) -> Result<(), failure::Error> {
        unimplemented!();
    }
}

