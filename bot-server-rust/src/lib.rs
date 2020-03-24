#![allow(clippy::needless_return)]

#[cfg(unix)]
#[macro_use]
extern crate shell;
#[macro_use]
extern crate enum_methods;

pub mod ast;
mod bot_server;
pub mod cli_auto_mode;
pub mod client_mode;
mod cmd;
mod common_ext;
pub mod protocol;
pub mod server_mode;
mod util;
