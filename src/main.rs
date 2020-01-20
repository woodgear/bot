use ws;
use encoding;
use ws::{listen, Handler, Sender, Message, Handshake, CloseCode, Error};
use failure;
use std::process::{Command,ExitStatus};
use std::error::Error as StdError;
mod common_ext;
use common_ext::*;
use std::process::Output;

#[derive(Eq,PartialEq)]
enum CurrentEncoding {
    GBK,
    UTF8
}

fn get_current_encoding()->Result<CurrentEncoding,failure::Error> {

    let out = cmd_raw("chcp")?;
    if out.status.success() {
        let out = String::from_utf8_lossy(&out.stdout).to_string();
        if out.contains("936") {
            return Ok(CurrentEncoding::GBK);
        } else {
            return Ok(CurrentEncoding::UTF8);
        }
    }
    return Err(failure::err_msg("get_current_encoding chcp fail"));
}

fn decoding_string(data:&Vec<u8>) ->Result<String,failure::Error> {
    use encoding::{Encoding, DecoderTrap};
    use encoding::all::GBK;
    
    if  get_current_encoding()? == CurrentEncoding::GBK {
        return GBK.decode(data, DecoderTrap::Strict).map_err(|e|failure::format_err!("decoding fail {}",e.to_string()));
    }
    return Ok(String::from_utf8_lossy(data).to_string());
}
fn cmd(arg:&str) -> Result<String,failure::Error> {
    let out  = cmd_raw(arg)?;
    if out.status.success() {
        return decoding_string(&out.stdout);
    }
    return decoding_string(&out.stderr);
}
fn cmd_raw(arg:&str) -> Result<Output,failure::Error> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg(arg)
                .output()?          
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(arg)
                .output()?
    };
    return Ok(output);
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<(),ws::Error> {
        println!("client connect");
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<(),ws::Error> {
        println!("msg is {}",msg);
        let out = cmd(&msg.into_text()?).to_ws_err()?;
        self.out.send(out)?;
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}




fn main() {
    println!("start server");
    listen("0.0.0.0:3012", |out| Server { out: out } ).unwrap()
  } 