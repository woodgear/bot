use crate::ast::*;
use crate::common_ext::*;
use crate::protocol::*;
use crate::util::*;
use log::*;
use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender};

struct Client {
    out: Sender,
}
pub fn send_cli_ast_to_server_send(cmd: String, out: &Sender) -> Result<(), failure::Error> {
    let cli_ast = CliAst::from_str(&cmd)?;
    let server_ast = cli_ast.to_server_ast()?;
    let buff = server_ast.into_binary()?;
    out.send(Message::Binary(buff))?;
    Ok(())
}
pub fn read_and_send(out: &Sender) -> Result<(), failure::Error> {
    let cmd = read_from_stdin()?;
    send_cli_ast_to_server_send(cmd, out)?;
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

pub fn client(url: String) {
    init_log();
    info!("try to connect to server {}", url);
    connect(url, |out| Client { out }).unwrap();
    // connect("ws://192.168.2.107:3012", |out| Client { out }).unwrap();
}
