use crate::ast::*;
use crate::common_ext::*;
use crate::protocol::*;
use crate::util::*;
use log::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Sender as WsSender};

struct Client {
    out: WsSender,
    stop: StopSignal,
}
pub fn send_cli_ast_to_server_send(cmd: String, out: &WsSender) -> Result<(), failure::Error> {
    let cli_ast = CliAst::from_str(&cmd)?;
    let server_ast = cli_ast.to_server_ast()?;
    let buff = server_ast.into_binary()?;
    out.send(Message::Binary(buff))?;
    Ok(())
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        info!("connect to server");
        if let Err(e) = self.read_and_send() {
            error!("err {:?}", e);
        }
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        self.handle_msg(msg);
        Ok(())
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
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct StopSignal {
    stop_signal_receiver: Arc<Mutex<Receiver<()>>>,
}

impl StopSignal {
    fn new(receiver: Receiver<()>) -> Self {
        Self {
            stop_signal_receiver: Arc::new(Mutex::new(receiver)),
        }
    }
    fn take(&mut self) -> bool {
        let ret = self.stop_signal_receiver.lock().unwrap().try_recv();
        return ret.is_ok();
    }
}

impl Client {
    pub fn read_and_send(&mut self) -> Result<(), failure::Error> {
        fn _read_and_send(out: &WsSender) -> Result<(), failure::Error> {
            let cmd = read_from_stdin()?;
            send_cli_ast_to_server_send(cmd, out)?;
            Ok(())
        }
        while let Err(e) = _read_and_send(&self.out) {
            if self.stop.take() {
                info!("reckon stop");

                self.out.close(ws::CloseCode::Normal).unwrap();
                return Ok(());
            } else {
                info!("no stop");
            }
            error!("err: {:?}", e);
        }
        Ok(())
    }
    fn handle_msg(&mut self, msg: Message) -> Result<(), failure::Error> {
        let msg = msg.into_text()?;
        let msg = serde_json::from_str::<ServerResponse>(&msg)?;
        match msg {
            ServerResponse::CallResult(r) => {
                println!(
                    "success: {}\noutput:\n---\n{}\n---",
                    r.status == 0,
                    r.output
                );
                self.read_and_send();
                return Ok(());
            }
            ServerResponse::SpawnResult(r) => {
                if r.status == 0 {
                    println!("success");
                } else {
                    println!("err:\n---\n{}\n---", r.err_msg);
                }
                self.read_and_send();
                return Ok(());
            }
            ServerResponse::CopyResult(r) => {
                if r.status {
                    println!("success");
                } else {
                    println!("err:\n---\n{}\n---", r.err_msg);
                }
                self.read_and_send();
                return Ok(());
            }
            ServerResponse::VersionResult(v) => {
                println!("{}", v);
                self.read_and_send();
                return Ok(());
            }
            ServerResponse::TailResult(t) => match t {
                TailResult::Err(e) => {
                    error!("tail err: {}", e);
                    self.read_and_send();
                    return Ok(());
                }
                TailResult::TailTimeout => {
                    info!("tail timeout");
                }
                TailResult::TailContinue(line) => {
                    info!("tail line {}", line);
                    return Ok(());
                }
                TailResult::TailEnd => {
                    info!("tail end");
                    self.read_and_send();
                    return Ok(());
                }
            },
            _ => {
                info!("not support yet");
            }
        }
        Ok(())
    }
}

pub fn client(url: String) {
    init_log();
    info!("try to connect to server {}", url);
    let (s, r) = channel();
    ctrlc::set_handler(move || {
        info!("stop");
        s.send(());
    })
    .expect("Error setting Ctrl-C handler");
    let stop_s = StopSignal::new(r);
    connect(url, move |out| Client {
        out,
        stop: stop_s.clone(),
    })
    .unwrap();
    // connect("ws://192.168.2.107:3012", |out| Client { out }).unwrap();
}
