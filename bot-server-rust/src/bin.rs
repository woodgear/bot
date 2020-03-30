use bot::{cli_auto_mode, client_mode, server_mode};

use structopt::StructOpt;
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
fn main() {
    let config = Config::from_args();
    match config.sub {
        SubCmd::Server { port } => {
            server_mode::server(port);
        }
        SubCmd::Client { url } => {
            client_mode::client(url);
        }
        SubCmd::Auto { url, cmd } => {
            cli_auto_mode::auto(url, cmd);
        }
    }
}