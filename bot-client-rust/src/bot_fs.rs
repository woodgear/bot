use crate::util::one_shot;
use bot::{ast::*, protocol::*};
use std::path::Path;
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

    pub async fn assign_dir(&self)->Result<String,failure::Error> {
        let res = one_shot(&self.server, ServerAst::AssignDir(AssignDirReq{})).await?;
        let res = res.into_assign_dir_result();
        return Ok(res.path);
    }

    pub async fn copy_dir<L:AsRef<Path>,R:AsRef<Path>>(&self, local_dir: L, remote:R) -> Result<(), failure::Error> {
        let local_dir = local_dir.as_ref().to_string_lossy().to_string(); 
        let remote = remote.as_ref().to_string_lossy().to_string();

        let tmp_dir = tempdir::TempDir::new("temp-zip_dir")?;
        let zip_path = tmp_dir.path().join("temp.zip");
        declare_fs::zip_dir(&local_dir,&zip_path)?;
        let zip_bin = std::fs::read(zip_path)?;
        let req = CopyDirServer{from:local_dir.clone(),to:remote.clone(),data:zip_bin};
        let res = one_shot(&self.server, ServerAst::CopyDir(req)).await?;
        let res = res.into_copy_dir_result();
        println!("{:?}",res);
        Ok(())
    }
}

