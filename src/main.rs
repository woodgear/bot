#![allow(clippy::needless_return)]

use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender};
mod common_ext;
use common_ext::*;
mod cmd;
use log::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
mod util;
#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileCli {
    from: String,
    to: String,
}
/// the user input
#[derive(Debug, PartialEq, Eq)]
enum CliAst {
    //execute and wait output
    Call(String),
    //spawn
    Spawn(String),
    CopyFile(CopyFileCli),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct CopyFileServer {
    from: String,
    to: String,
    data: Vec<u8>,
    md5: String,
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
enum ServerAst {
    Call(String),
    Spawn(String),
    CopyFile(CopyFileServer),
}

#[derive(Serialize, PartialEq, Eq, Deserialize, Debug)]
struct ServerAstResponse {
    success: bool,
    ret: String,
}
type PickFnHandles<T> = Vec<(&'static str, Box<dyn FnOnce(String) -> T>)>;
fn pick<T>(msg: &str, fs: PickFnHandles<T>) -> Result<T, failure::Error> {
    for (prefix, f) in fs.into_iter() {
        if msg.starts_with(prefix) {
            let (_, data) = msg.split_at(prefix.len());
            //means that the msg is ${prefix}${data} and that is invalid
            if data.len() == data.trim_start().len() {
                return Err(failure::format_err!("prefix should end with a whitespace"));
            }
            let data = data.trim();
            return Ok(f(data.to_string()));
        }
    }
    return Err(failure::format_err!("could not match any of prefix"));
}

impl CliAst {
    fn from_str(msg: &str) -> Result<Self, failure::Error> {
        info!("cliast {:?}", msg);
        let ast = pick(
            msg,
            vec![
                (
                    "call",
                    Box::new(|data: String| {
                        return CliAst::Call(data);
                    }),
                ),
                (
                    "spawn",
                    Box::new(|data: String| {
                        return CliAst::Spawn(data);
                    }),
                ),
                (
                    "copy-file",
                    Box::new(|data: String| {
                        info!("copy-file {}", data);
                        let cp: CopyFileCli = serde_json::from_str(&data).unwrap();
                        return CliAst::CopyFile(cp);
                    }),
                ),
            ],
        );
        return ast;
    }
}

impl CliAst {
    fn to_server_ast(&self) -> Result<ServerAst, failure::Error> {
        let ret = match self {
            CliAst::Call(arg) => ServerAst::Call(arg.to_string()),
            CliAst::Spawn(arg) => ServerAst::Spawn(arg.to_string()),
            CliAst::CopyFile(cp) => {
                let file_buffer = std::fs::read(&cp.from)?;
                let md5 = md5::compute(&file_buffer);
                ServerAst::CopyFile(CopyFileServer {
                    from: cp.from.to_string(),
                    to: cp.to.to_string(),
                    data: file_buffer,
                    md5: format!("{:?}",md5),
                })
            }
        };
        return Ok(ret);
    }
}

impl ServerAst {
    fn into_binary(self) -> Result<Vec<u8>, failure::Error> {
        let buff = bincode::serialize(&self)?;
        return Ok(buff);
    }
    fn from_binary(buff: &[u8]) -> Result<ServerAst, failure::Error> {
        let this: ServerAst = bincode::deserialize(buff)?;
        Ok(this)
    }

    fn do_stuff(&self) -> String {
        let ret = || -> Result<String, failure::Error> {
            match self {
                ServerAst::Call(data) => {
                    info!("server call {}", data);
                    let ret = cmd::exec(format!("cmd /c {}", data))?;
                    return Ok(ret);
                }
                ServerAst::Spawn(data) => {
                    info!("server spawn {}", data);
                    cmd::exec_without_wait(format!("cmd /c {}", data))?;
                    return Ok("".to_string());
                }
                ServerAst::CopyFile(cp) => {
                    info!(
                        "server copy-file from {} to {} len {} md5 {}",
                        cp.from,
                        cp.to,
                        cp.data.len(),
                        cp.md5
                    );
                    let buff_md5 = util::md5(&cp.data);
                    if buff_md5!= cp.md5 {
                        return Err(failure::format_err!("copy-file fail check buff md5 fail expect {} find {}",cp.md5,buff_md5));
                    }
                    std::fs::write(&cp.to, &cp.data)?;
                    let file_md5 = util::md5_file(&cp.to)?; 
                    if file_md5 != buff_md5 {
                        return Err(failure::format_err!("copy-file fail check file md5 fail expect {} find {}",buff_md5,file_md5));
                    }
                    return Ok("".to_owned());
                }
            }
        }();
        match ret {
            Ok(data) => {
                return format!("success\n{}", data);
            }
            Err(e) => {
                return format!("error\n{}", e.to_string());
            }
        }
    }
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        info!("client connect");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let ret = ServerAst::from_binary(&msg.into_data()).map(|a| a.do_stuff());
        let server_response = match ret {
            Ok(msg) => ServerAstResponse {
                success: true,
                ret: msg,
            },
            Err(e) => ServerAstResponse {
                success: false,
                ret: e.to_string(),
            },
        };
        let server_response_json = serde_json::to_string(&server_response).unwrap();
        self.out.send(server_response_json)?;
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => info!("The client is done with the connection."),
            CloseCode::Away => info!("The client is leaving the site."),
            _ => error!("The client encountered an error: {}", reason),
        }
    }
}

fn init_log() {
    use simplelog::*;
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();
}

#[derive(StructOpt, Debug)]
#[structopt(name = "bot")]
struct Config {
    #[structopt(subcommand)]
    sub: SubCmd,
}

#[derive(StructOpt, Debug, Eq, PartialEq)]
#[structopt(version = "0.1", author = "fwdx")]
pub enum SubCmd {
    Server { port: u32 },
    Client { url: String },
    Auto { url: String, cmd: String },
}

fn server(port: u32) {
    init_log();
    info!("start server");
    let url = format!("0.0.0.0:{}", port);
    if let Err(e) = listen(url, |out| Server { out }) {
        error!("{:?}", e);
    }
}

struct Client {
    out: Sender,
}

fn read_from_stdin() -> Result<String, failure::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    return Ok(input);
}
fn read_and_send(out: &Sender) -> Result<(), failure::Error> {
    let cmd = read_from_stdin()?;
    send_cli_ast_to_server_send(cmd, out)?;
    Ok(())
}
// copy-file {"from":"C:\Users\18754\Desktop\a.png","to":"C:\Users\runa\Desktop\a.png"}
fn send_cli_ast_to_server_send(cmd: String, out: &Sender) -> Result<(), failure::Error> {
    let cli_ast = CliAst::from_str(&cmd)?;
    let server_ast = cli_ast.to_server_ast()?;
    let buff = server_ast.into_binary()?;
    out.send(Message::Binary(buff))?;
    Ok(())
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        info!("connect to server");
        read_and_send(&self.out).to_ws_err()
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        info!("reveive form server\n{}\n", msg);
        read_and_send(&self.out).to_ws_err()
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => info!("The client is done with the connection."),
            CloseCode::Away => info!("The client is leaving the site."),
            _ => error!("The client encountered an error: {}", reason),
        }
        self.out.close(ws::CloseCode::Normal).unwrap();
    }
}

fn client(url: String) {
    init_log();
    info!("try to connect to server {}", url);
    connect(url, |out| Client { out }).unwrap();
    // connect("ws://192.168.2.107:3012", |out| Client { out }).unwrap();
}

struct Auto {
    ws_sender: Sender,
    cmd: String,
    msg_sender: std::sync::mpsc::Sender<ServerAstResponse>,
}

impl Handler for Auto {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        send_cli_ast_to_server_send(self.cmd.to_string(), &self.ws_sender).to_ws_err()
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let msg = msg.into_text()?;
        let ret: ServerAstResponse = serde_json::from_str(&msg).unwrap();
        self.msg_sender.send(ret).unwrap();
        self.ws_sender.close(ws::CloseCode::Normal)?;
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => info!("The client is done with the connection."),
            CloseCode::Away => info!("The client is leaving the site."),
            _ => error!("The client encountered an error: {}", reason),
        }
        self.ws_sender.close(ws::CloseCode::Normal).unwrap();
    }
}

fn one_shot(url: String, cmd: String) -> Result<ServerAstResponse, failure::Error> {
    use std::sync::mpsc::channel;
    let (sender, receiver) = channel::<ServerAstResponse>();

    connect(url, move |out| Auto {
        ws_sender: out,
        msg_sender: sender.clone(),
        cmd: cmd.clone(),
    })?;
    let msg = receiver.recv()?;
    return Ok(msg);
}

fn auto(url: String, cmd: String) {
    println!("auto");
    init_log();

    let msg = one_shot(url, cmd).unwrap();
    info!("{:?}", msg);
}

fn main() {
    let config = Config::from_args();
    match config.sub {
        SubCmd::Server { port } => {
            server(port);
        }
        SubCmd::Client { url } => {
            client(url);
        }
        SubCmd::Auto { url, cmd } => {
            auto(url, cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ast() {
        let cmd = "call xxx";
        let ast = CliAst::from_str(cmd);
        assert_eq!(CliAst::Call("xxx".to_string()), ast.unwrap());
        let cmd = "spawn xxx";
        let ast = CliAst::from_str(cmd);
        assert_eq!(CliAst::Spawn("xxx".to_string()), ast.unwrap());
        let cmd = "spawn";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "call";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "callxxx";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "spawnxxx";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
    }
    #[test]
    fn test_pick() {
        let ast = pick(
            "copy-file {\"from\":\"C:\\\\a  b\\\\c.txt\",\"to\":\"D:\\\\a  b\\\\c.txt\"}",
            vec![(
                "copy-file",
                Box::new(|data: String| {
                    #[derive(Serialize, Deserialize, Debug)]
                    pub struct CopyFileCli {
                        from: String,
                        to: String,
                    };
                    let val: CopyFileCli = serde_json::from_str(&data).unwrap();
                    println!("{:?}", val);
                    println!("{}", val.from);
                    return data;
                }),
            )],
        )
        .unwrap();
        println!("{}", ast);
    }
    #[ignore]
    #[test]
    fn test_cmd() {
        let res = cmd::exec("git status").unwrap();
        println!("res\n {}", res);
    }
}
