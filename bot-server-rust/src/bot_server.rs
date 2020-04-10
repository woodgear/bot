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
    fn call(&self, config: ExecConfig) -> CallResult {
        return std::thread::spawn(move || {
            let cwd = std::env::current_dir().unwrap();
            if !config.cwd.is_empty() {
                let cwd = std::path::Path::new(&config.cwd);
                if !(cwd.exists() && cwd.is_dir()) {
                    return CallResult {
                        status: -1,
                        output: format!("{:?} is not exist or is not a dir", cwd),
                    };
                }
                std::env::set_current_dir(config.cwd);
            }

            let res = cmd::exec(format!("{}", config.cmd));
            std::env::set_current_dir(cwd).unwrap();
            match res {
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
        })
        .join()
        .unwrap();
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
                    status: -1,
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

    fn copy_dir(&self, cp: CopyDirServer) -> CopyDirResult {
        fn do_cp(cp: CopyDirServer) -> Result<(), failure::Error> {
            let tmp_dir = tempdir::TempDir::new("temp-zip_dir")?;
            let zip_path = tmp_dir.path().join("temp.zip");
            std::fs::write(&zip_path, cp.data)?;
            declare_fs::unzip(&zip_path, cp.to)?;
            Ok(())
        }

        if let Err(e) = do_cp(cp) {
            return CopyDirResult {
                status: false,
                err_msg: e.to_string(),
            };
        }
        return CopyDirResult {
            status: true,
            err_msg: "".to_owned(),
        };
    }

    fn vresion(&self) -> String {
        return "0.0.1".to_string();
    }
}
