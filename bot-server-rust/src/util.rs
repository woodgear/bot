use ws::Sender;
pub fn init_log() {
    use simplelog::*;
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();
}

pub fn md5_file<P: AsRef<std::path::Path>>(p: P) -> Result<String, failure::Error> {
    let buff = std::fs::read(p)?;
    return Ok(md5(&buff));
}

pub fn md5(buff: &[u8]) -> String {
    let digest = md5::compute(buff);
    return format!("{:?}", digest);
}

pub fn read_from_stdin() -> Result<String, failure::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    return Ok(input);
}
use std::path::{Path, PathBuf};
