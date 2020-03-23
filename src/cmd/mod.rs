#[cfg(windows)]
mod win_cmd;

#[cfg(windows)]
pub mod windows {
    pub fn exec(cmd: String) -> Result<String, failure::Error> {
        win_cmd::exec(format!("cmd /c {}", cmd))
    }

    pub fn spawn(cmd: String) -> Result<(), failure::Error> {
        win_cmd::exec_without_wait(format!("cmd /c {}", cmd))
    }
}

#[cfg(windows)]
pub use windows::*;

#[cfg(unix)]
pub mod unix {
    pub fn exec(cmd: String) -> Result<String, failure::Error> {
        use shell::cmd;
        let out = cmd!(&cmd).stdout_utf8().unwrap();
        Ok(out)
    }

    pub fn spawn(cmd: String) -> Result<(), failure::Error> {
        use shell::cmd;
        let _ = cmd!(&cmd).spawn().unwrap();
        Ok(())
    }
}

#[cfg(unix)]
pub use unix::*;
