use failure;
use ws;
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};
mod common_ext;
use common_ext::*;
mod cmd;

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
        println!("client connect");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        println!("msg is {}", msg);
        let ret = Ast::from_str(&msg.into_text()?)
            .map(|a| a.do_stuff())
            .unwrap_or_else(|e| format!("error\n{}", e));
        self.out.send(ret)?;
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
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

fn main() {
    println!("start server");
    listen("0.0.0.0:3012", |out| Server { out: out }).unwrap()
}
