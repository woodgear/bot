use std::path::*;
// pub struct MockClient {
//     fs:MockFs,
//     tail:MockTail,
//     config:String,
// }

pub struct FsClient {
    server: String,
}

pub enum BotPath {
    Local(PathBuf),
    Remote(PathBuf),
}
pub enum BotError {
    Timeout,
    ServerErr(failure::Error),
    Other(failure::Error),
}

pub type BotResult<T> = std::result::Result<T, BotError>;

// pub trait BotFs {
//     /// read file from remote
//     fn read_file(&self, path: BotPath) -> Result<Vec<u8>>;

//     /// copy local file to remote
//     fn copy_file<P, Q>(&self, from: BotPath, to: BotPath) -> Result<()>;

//     /// detect is path is dir in remote
//     fn is_dir(&self, path: BotPath) -> bool;

//     /// detect is file is dir in remote
//     fn is_file(&self, path: BotPath) -> bool;
//     /// create dir in remote
//     fn create_dir(&self, path: BotPath) -> Result<()>;
//     /// create dir all in remote
//     fn create_dir_all(&self, path: BotPath) -> Result<()>;
//     /// remove remote dir
//     fn remove_dir(&self, path: BotPath) -> Result<()>;
//     /// remove remote dir all
//     fn remove_dir_all(&self, path: BotPath) -> Result<()>;
//     /// copy local dir to remote
//     fn copy_dir(&self, path: BotPath) -> Result<()>;
//     fn read_file(&self,path:BotPath) ->Result<Vec<u8>>
// }

struct BotFsImpl {

}

impl BotFsImpl {
   async fn read_file_on_shot(&self,path:BotPath) -> Result<Vec<u8>,failure::Error>  {
       Ok(vec![])
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use tokio::prelude::*;
   use tokio::runtime::Builder;

    fn run_one<F>(f: F) -> Result<F::Item, F::Error>
    where
        F: IntoFuture,
        F::Future: Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + 'static,
    {
        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        runtime.block_on(f.into_future())
    }

    async fn async_test_bot_fs() -> bool {
        false
    }
    #[test]
    fn test_bot_fs() {
        let f  = async_test_bot_fs();
        let ret = run_one(f).unwrap();
        assert!(ret)
    }
}
// struct ShellConfig {

// }

// pub trait BotShell {
//     fn exec(&self,config:ShellConfig)-> Result<String>;
//     fn spawn(&self,config:ShellConfig)-> Result<u32>;
// }

// pub trait BotTail {
//     fn tail(&self,path:BotPath) ;
// }