use serde::{Deserialize, Serialize};
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileCli {
    pub from: String,
    pub to: String,
}
/// the user input
#[derive(Debug, PartialEq, Eq)]
pub enum CliAst {
    //execute and wait output
    Call(String),
    //spawn
    Spawn(String),
    CopyFile(CopyFileCli),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileServer {
    pub from: String,
    pub to: String,
    pub data: Vec<u8>,
    pub md5: String,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub enum ServerAst {
    Call(String),
    Spawn(String),
    CopyFile(CopyFileServer),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CallResult {
    pub status: i32,
    pub output: String,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct SpawnResult {
    pub status: i32,
    pub err_msg: String,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyResult {
    pub status: bool,
    pub err_msg: String,
}

pub trait BotServer {
    fn call(&self, cmd: String) -> CallResult;
    fn spawn(&self, cmd: String) -> SpawnResult;
    fn copy(&self, ast: CopyFileServer) -> CopyResult;
    fn vresion(&self) -> String;
}
