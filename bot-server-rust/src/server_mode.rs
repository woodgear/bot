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
                let ret = ServerResponse::CallRet(ret);
                let ret_json = serde_json::to_string(&ret).unwrap();
                self.out.send(ret_json);
            }
            ServerAst::Spawn(data) => {
                let ret = bot_server_impl.spawn(data);
                let ret = ServerResponse::SpawnRet(ret);
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
                let ret = ServerResponse::CopyRet(ret);
                let ret_json = serde_json::to_string(&ret).unwrap();
                self.out.send(ret_json);
            }
            ServerAst::Tail(path) => {
                use futures::{future::Future, stream::Stream};
                use tail_rust::Tail;
                use tokio::prelude::*;
                use tokio_threadpool::ThreadPool;
                let ws_sender = self.out.clone();

                let h = std::thread::spawn(move || {
                    let ws_sender = ws_sender.clone();
                    let ws_sender_1 = ws_sender.clone();
                    tokio::run(futures::lazy(move || {
                        for e in Tail::new(&path)
                            .unwrap()
                            .timeout(std::time::Duration::from_secs(3))
                            .wait()
                        {
                            match e {
                                Ok(line) => {
                                    println!("line ==> {:?}", line);
                                    let msg = ServerResponse::TailResult(TailResult::TailContinue(
                                        line.clone(),
                                    ));
                                    let msg_json = serde_json::to_string(&msg).unwrap();
                                    ws_sender.send(msg_json);
                                    println!("line {}", line);
                                }
                                Err(e) => {
                                    if !e.is_inner() {
                                        println!("tail timeout");
                                        let msg =
                                            ServerResponse::TailResult(TailResult::TailTimeout);
                                        let msg_json = serde_json::to_string(&msg).unwrap();
                                        ws_sender.send(msg_json);
                                    } else {
                                        println!("tail err {:?}", e);
                                        let msg = ServerResponse::TailResult(TailResult::TailEnd);
                                        let msg_json = serde_json::to_string(&msg).unwrap();
                                        ws_sender.send(msg_json);
                                    }
                                }
                            }
                        }
                        Ok(())
                    }));
                    let msg = ServerResponse::TailResult(TailResult::TailEnd);
                    let msg_json = serde_json::to_string(&msg).unwrap();
                    ws_sender_1.send(msg_json);
                    println!("{:?}", "end");
                });
                // h.join().unwrap();
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
