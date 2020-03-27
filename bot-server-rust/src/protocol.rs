use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileCli {
    pub from: String,
    pub to: String,
}
/// the user input
#[derive(EnumIntoGetters, EnumAsGetters, EnumIsA, Debug, PartialEq, Eq)]
pub enum CliAst {
    //execute and wait output
    Call(String),
    //spawn
    Spawn(String),
    CopyFile(CopyFileCli),
    Tail(String),
}
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileServer {
    pub from: String,
    pub to: String,
    pub data: Vec<u8>,
    pub md5: String,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyDirServer {
    pub from: String,
    pub to: String,
    pub data: Vec<u8>,
}

#[derive(EnumIntoGetters, EnumAsGetters, EnumIsA, Serialize, PartialEq, Eq, Deserialize, Debug)]
pub enum ServerAst {
    Call(String),
    Spawn(String),
    Tail(String),
    CopyFile(CopyFileServer),
    CopyDir(CopyDirServer),
    ReadFile(String),
    WriteFile(WriteFileReq),
    AssignDir(AssignDirReq),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct WriteFileReq {
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct AssignDirReq {}

#[derive(EnumIntoGetters, EnumAsGetters, EnumIsA, Serialize, PartialEq, Eq, Deserialize, Debug)]
pub enum ServerResponse {
    CallResult(CallResult),
    SpawnResult(SpawnResult),
    CopyResult(CopyResult),
    CopyDirResult(CopyDirResult),
    VersionResult(String),
    TailResult(TailResult),
    ReadFileResult(ReadFileResult),
    WriteFileResult(WriteFileResult),
    AssignDirResult(AssignDirResult),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct ReadFileResult(pub Result<Vec<u8>, String>);

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct WriteFileResult(pub Result<(), String>);

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct AssignDirResult {
    pub path: String,
}

#[derive(EnumIntoGetters, EnumAsGetters, EnumIsA, Serialize, PartialEq, Eq, Deserialize, Debug)]
pub enum TailResult {
    Err(String),
    TailContinue(String),
    TailTimeout,
    TailEnd,
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
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyDirResult {
    pub status: bool,
    pub err_msg: String,
}

pub trait BotServer {
    fn call(&self, cmd: String) -> CallResult;
    fn spawn(&self, cmd: String) -> SpawnResult;
    fn copy(&self, ast: CopyFileServer) -> CopyResult;
    fn copy_dir(&self, ast: CopyDirServer) -> CopyDirResult;
    fn vresion(&self) -> String;
}
