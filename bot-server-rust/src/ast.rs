use crate::protocol::*;
use log::*;

type PickFnHandles<T> = Vec<(&'static str, Box<dyn FnOnce(String) -> T>)>;

pub fn pick<T>(msg: &str, fs: PickFnHandles<T>) -> Result<T, failure::Error> {
    for (prefix, f) in fs.into_iter() {
        if msg.starts_with(prefix) {
            let (_, data) = msg.split_at(prefix.len());
            //means that the msg is ${prefix}${data} and that is invalid
            if data.len() == data.trim_start().len() {
                return Err(failure::format_err!("prefix should end with a whitespace"));
            }
            let data = data.trim();
            return Ok(f(data.to_string()));
        }
    }
    return Err(failure::format_err!("could not match any of prefix"));
}

impl CliAst {
    pub fn from_str(msg: &str) -> Result<Self, failure::Error> {
        info!("cliast {:?}", msg);
        let ast = pick(
            msg,
            vec![
                (
                    "call",
                    Box::new(|data: String| {
                        return CliAst::Call(data);
                    }),
                ),
                (
                    "spawn",
                    Box::new(|data: String| {
                        return CliAst::Spawn(data);
                    }),
                ),
                (
                    "copy-file",
                    Box::new(|data: String| {
                        info!("copy-file {}", data);
                        let cp: CopyFileCli = serde_json::from_str(&data).unwrap();
                        return CliAst::CopyFile(cp);
                    }),
                ),
                (
                    "tail",
                    Box::new(|data: String| {
                        info!("tail {}", data);
                        return CliAst::Tail(data);
                    }),
                ),
            ],
        );
        return ast;
    }
}

impl CliAst {
    pub fn to_server_ast(&self) -> Result<ServerAst, failure::Error> {
        let ret = match self {
            CliAst::Call(arg) => ServerAst::Call(ExecConfig {
                cmd: arg.to_string(),
                cwd: "".to_string(),
            }),
            CliAst::Spawn(arg) => ServerAst::Spawn(arg.to_string()),
            CliAst::CopyFile(cp) => {
                let file_buffer = std::fs::read(&cp.from)?;
                let md5 = md5::compute(&file_buffer);
                ServerAst::CopyFile(CopyFileServer {
                    from: cp.from.to_string(),
                    to: cp.to.to_string(),
                    data: file_buffer,
                    md5: format!("{:?}", md5),
                })
            }
            CliAst::Tail(path) => ServerAst::Tail(path.to_owned()),
        };
        return Ok(ret);
    }
}

impl ServerAst {
    pub fn into_binary(self) -> Result<Vec<u8>, failure::Error> {
        let buff = bincode::serialize(&self)?;
        return Ok(buff);
    }
    pub fn from_binary(buff: &[u8]) -> Result<ServerAst, failure::Error> {
        let this: ServerAst = bincode::deserialize(buff)?;
        Ok(this)
    }
}

impl ServerResponse {
    pub fn from_str(msg: &String) -> Result<Self, failure::Error> {
        let this: Self = serde_json::from_str(msg)?;
        Ok(this)
    }
    pub fn into_binary(&self) -> Result<Vec<u8>, failure::Error> {
        let buff = bincode::serialize(&self)?;
        return Ok(buff);
    }
    pub fn from_binary(buff: &Vec<u8>) -> Result<Self, failure::Error> {
        let this: Self = bincode::deserialize(buff)?;
        Ok(this)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    #[test]
    fn test_ast() {
        let cmd = "call xxx";
        let ast = CliAst::from_str(cmd);
        assert_eq!(CliAst::Call("xxx".to_string()), ast.unwrap());
        let cmd = "spawn xxx";
        let ast = CliAst::from_str(cmd);
        assert_eq!(CliAst::Spawn("xxx".to_string()), ast.unwrap());
        let cmd = "spawn";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "call";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "callxxx";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
        let cmd = "spawnxxx";
        let ast = CliAst::from_str(cmd);
        assert!(ast.is_err());
    }
    #[test]
    fn test_pick() {
        let ast = pick(
            "copy-file {\"from\":\"C:\\\\a  b\\\\c.txt\",\"to\":\"D:\\\\a  b\\\\c.txt\"}",
            vec![(
                "copy-file",
                Box::new(|data: String| {
                    #[derive(Serialize, Deserialize, Debug)]
                    pub struct CopyFileCli {
                        from: String,
                        to: String,
                    };
                    let val: CopyFileCli = serde_json::from_str(&data).unwrap();
                    println!("{:?}", val);
                    println!("{}", val.from);
                    return data;
                }),
            )],
        )
        .unwrap();
        println!("{}", ast);
    }
    #[ignore]
    #[test]
    fn test_cmd() {
        use crate::cmd;
        let res = cmd::exec("git status".to_owned()).unwrap();
        println!("res\n {}", res);
    }
}
