use ws;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

use std::process::{Command,ExitStatus};

fn cmd(arg:String) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .arg("/C")
                .arg(arg)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(arg)
                .output()
                .expect("failed to execute process")
    };
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout).to_string();
        return Ok(format!("success\n{}\n",out));
    } else {
        let out = String::from_utf8_lossy(&output.stderr).to_string();
        return Ok(format!("success\n{}\n",out));
    }
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("client connect");
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("msg is {}",msg);
        let out = cmd(msg.into_text()?)?;
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
    listen("127.0.0.1:3012", |out| Server { out: out } ).unwrap()
  } 