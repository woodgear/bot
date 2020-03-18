use crate::bot_server::*;
use crate::common_ext::*;
use crate::protocol::*;
use crate::util::*;
use log::*;
use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender};

pub fn server(port: u32) {
    init_log();
    info!("start server");
    let url = format!("0.0.0.0:{}", port);
    if let Err(e) = listen(url, |out| Server { out }) {
        error!("{:?}", e);
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
        let ast = ServerAst::from_binary(&msg.into_data()).to_ws_err()?;
        let bot_server_impl = BotServerImpl::new();
        match ast {
            ServerAst::Call(data) => {
                let ret = bot_server_impl.call(data);
                let ret_json = serde_json::to_string(&ret).unwrap();
                self.out.send(ret_json);
            }
            ServerAst::Spawn(data) => {
                let ret = bot_server_impl.spawn(data);
                let ret_json = serde_json::to_string(&ret).unwrap();
                self.out.send(ret_json);
            }
            ServerAst::CopyFile(cp) => {
                info!(
                    "server copy-file from {} to {} len {} md5 {}",
                    cp.from,
                    cp.to,
                    cp.data.len(),
                    cp.md5
                );
                let ret = bot_server_impl.copy(cp);
                let ret_json = serde_json::to_string(&ret).unwrap();
                self.out.send(ret_json);
            }
        }
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
