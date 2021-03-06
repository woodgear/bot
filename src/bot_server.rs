use crate::cmd;
use crate::protocol::*;
use crate::util;
use log::*;
pub struct BotServerImpl {}

impl BotServerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl BotServer for BotServerImpl {
    fn call(&self, cmd: String) -> CallResult {
        match cmd::exec(format!("{}", cmd)) {
            Ok(msg) => {
                return CallResult {
                    status: 0,
                    output: msg,
                }
            }
            Err(e) => {
                return CallResult {
                    status: -1,
                    output: e.to_string(),
                }
            }
        }
    }

    fn spawn(&self, cmd: String) -> SpawnResult {
        match cmd::spawn(format!("{}", cmd)) {
            Ok(msg) => {
                return SpawnResult {
                    status: 0,
                    err_msg: "".to_owned(),
                }
            }
            Err(e) => {
                return SpawnResult {
                    status: 0,
                    err_msg: e.to_string(),
                }
            }
        }
    }

    fn copy(&self, cp: CopyFileServer) -> CopyResult {
        info!(
            "server copy-file from {} to {} len {} md5 {}",
            cp.from,
            cp.to,
            cp.data.len(),
            cp.md5
        );
        fn do_cp(cp: CopyFileServer) -> Result<(), failure::Error> {
            let buff_md5 = util::md5(&cp.data);
            if buff_md5 != cp.md5 {
                return Err(failure::format_err!(
                    "copy-file fail check buff md5 fail expect {} find {}",
                    cp.md5,
                    buff_md5
                ));
            }
            std::fs::write(&cp.to, &cp.data)?;
            let file_md5 = util::md5_file(&cp.to)?;
            if file_md5 != buff_md5 {
                return Err(failure::format_err!(
                    "copy-file fail check file md5 fail expect {} find {}",
                    buff_md5,
                    file_md5
                ));
            }
            return Ok(());
        }
        if let Err(e) = do_cp(cp) {
            return CopyResult {
                status: false,
                err_msg: e.to_string(),
            };
        }
        return CopyResult {
            status: true,
            err_msg: "".to_owned(),
        };
    }

    fn vresion(&self) -> String {
        return "0.0.1".to_string();
    }
}
