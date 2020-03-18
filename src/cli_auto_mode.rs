use crate::ast::*;
use crate::ast::*;
use crate::common_ext::*;
use crate::protocol::*;
use crate::util::*;
use log::*;
use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender};
pub struct Auto {
    ws_sender: Sender,
    cmd: String,
    msg_sender: std::sync::mpsc::Sender<String>,
}

// copy-file {"from":"C:\Users\18754\Desktop\a.png","to":"C:\Users\runa\Desktop\a.png"}
pub fn send_cli_ast_to_server_send(cmd: String, out: &Sender) -> Result<(), failure::Error> {
    let cli_ast = CliAst::from_str(&cmd)?;
    let server_ast = cli_ast.to_server_ast()?;
    let buff = server_ast.into_binary()?;
    out.send(Message::Binary(buff))?;
    Ok(())
}

impl Handler for Auto {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        send_cli_ast_to_server_send(self.cmd.to_string(), &self.ws_sender).to_ws_err()
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        let msg = msg.into_text()?;
        self.msg_sender.send(msg).unwrap();
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

fn one_shot(url: String, cmd: String) -> Result<String, failure::Error> {
    use std::sync::mpsc::channel;
    let (sender, receiver) = channel::<String>();

    connect(url, move |out| Auto {
        ws_sender: out,
        msg_sender: sender.clone(),
        cmd: cmd.clone(),
    })?;
    let msg = receiver.recv()?;
    return Ok(msg);
}

pub fn auto(url: String, cmd: String) {
    println!("auto");
    init_log();

    let msg = one_shot(url, cmd).unwrap();
    info!("{:?}", msg);
}
