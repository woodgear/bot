use failure;
use ws;
use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender};
mod common_ext;
use common_ext::*;
mod cmd;
use log::*;
use structopt::StructOpt;

struct Server {
    out: Sender,
}

#[derive(Debug, PartialEq, Eq)]
enum Ast {
    //execute and wait output
    Call(String),
    //spawn
    Spawn(String),
}

fn pick<T>(msg: &str, fs: Vec<(&str, Box<dyn FnOnce(String) -> T>)>) -> Result<T, failure::Error> {
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

impl Ast {
    fn from_str(msg: &str) -> Result<Self, failure::Error> {
        let ast = pick(
            msg,
            vec![
                (
                    "call",
                    Box::new(|data: String| {
                        return Ast::Call(data);
                    }),
                ),
                (
                    "spawn",
                    Box::new(|data: String| {
                        return Ast::Spawn(data);
                    }),
                ),
            ],
        );
        return ast;
    }
}

impl Ast {
    fn do_stuff(&self) -> String {
        let ret = || -> Result<String, failure::Error> {
            match self {
                Ast::Call(data) => {
                    let ret = cmd::exec(format!("cmd /c {}", data))?;
                    return Ok(ret);
                }
                Ast::Spawn(data) => {
                    cmd::exec_without_wait(format!("cmd /c {}", data))?;
                    return Ok("".to_string());
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

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        info!("client connect");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        info!("msg is {}", msg);
        let ret = Ast::from_str(&msg.into_text()?)
            .map(|a| a.do_stuff())
            .unwrap_or_else(|e| format!("error\n{}", e));
        self.out.send(ret)?;
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
}

fn server(port: u32) {
    info!("start server");
    let url = format!("0.0.0.0:{}", port);
    if let Err(e) = listen(url, |out| Server { out: out }) {
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

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        info!("connect to server");
        let cmd = read_from_stdin().to_ws_err()?;
        self.out.send(cmd)?;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        info!("reveive form server\n{}\n", msg);
        let cmd = read_from_stdin().to_ws_err()?;
        self.out.send(cmd)?;
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

fn client(url: String) {
    connect(url, |out| Client { out }).unwrap();
}

fn main() {
    init_log();
    let config = Config::from_args();
    info!("config {:?}", config);
    match config.sub {
        SubCmd::Server { port } => {
            server(port);
        }
        SubCmd::Client { url } => {
            client(url);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ast() {
        let cmd = "call xxx";
        let ast = Ast::from_str(cmd);
        assert_eq!(Ast::Call("xxx".to_string()), ast.unwrap());
        let cmd = "spawn xxx";
        let ast = Ast::from_str(cmd);
        assert_eq!(Ast::Spawn("xxx".to_string()), ast.unwrap());
        let cmd = "spawn";
        let ast = Ast::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "call";
        let ast = Ast::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "callxxx";
        let ast = Ast::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "spawnxxx";
        let ast = Ast::from_str(cmd);
        assert!(ast.is_err());
    }

    #[ignore]
    #[test]
    fn test_cmd() {
        let res = cmd::exec("git status").unwrap();
        println!("res\n {}", res);
    }
}
