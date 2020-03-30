use async_std::stream::StreamExt;
use async_std::task;
use bot::protocol::ExecConfig;
use bot_client_rust::{prelude::*, ret_eq};
use std::path::PathBuf;
use std::time::Duration;
async fn init_package(pipeline_id: String) {}

async fn async_test_install() -> Result<bool, failure::Error> {
    let bot = BotClient::new("ws://127.0.0.1:12345");
    let fs = bot.fs();
    let bs = bot.shell();

    // let r_root = fs.assign_dir().await?;
    // let runtime = PathBuf::from(&r_root).join("runtime");
    let runtime =
        PathBuf::from(r#"C:\Users\developer\work\lab\bot\bot-server-rust\data\QhHFoSt\runtime"#);
    let runtime_str = runtime.to_string_lossy().to_string();
    let mut tail = bot
        .tail(&format!(r#"{}\log\3rt\edr-agent.log"#, runtime_str))
        .await?;
    ret_eq!(bs.service_exist("edrnpf").await?, false);
    ret_eq!(bs.service_exist("registry_driver").await?, false);
    ret_eq!(bs.service_exist("trantect-edr-file-system").await?, false);
    let cmd = format!(r#"{}\edr_agent.exe install"#, runtime_str);
    println!("{:?} {}", "start install", cmd);
    let res = futures::join!(
        StreamAssert::new(
            vec!["edr-client start install", "edr-client complete install"],
            vec!["Error"],
            "edr-client complete install",
            tail,
            1 * 30,
        ),
        bs.exec(ExecConfig {
            cmd,
            cwd: runtime_str.to_string(),
        }),
    );

    ret_eq!(res.0?,true,"should match log");

    println!("{:?}", "install over");
    ret_eq!(
        bs.service_exist("edrnpf").await?,
        true,
        "edrnpf should exist"
    );
    ret_eq!(
        bs.service_exist("trantect-edr-registry").await?,
        true,
        "reg should exist"
    );
    ret_eq!(
        bs.service_exist("trantect-edr-file-system").await?,
        true,
        "edr-file should exist"
    );

    //旧的服务名应当被删掉
    ret_eq!(bs.service_exist("registry_driver").await?, false);

    ret_eq!(
        bs.service_running("edrnpf").await?,
        false,
        "edrnpf should not running"
    );
    ret_eq!(
        bs.service_running("trantect-edr-registry").await?,
        false,
        "edr-registry should not running"
    );
    ret_eq!(
        bs.service_running("trantect-edr-file-system").await?,
        false,
        "edr-file should not running"
    );

    let cmd = format!(r#"{}\edr_agent.exe uninstall"#, runtime_str);
    println!("{:?} {}", "start uninstall", cmd);



    let res = bs.exec(ExecConfig {
        cmd,
        cwd: runtime_str.to_string(),
    }).await;
    println!("uninstall over res {:?}", res);
    ret_eq!(
        bs.service_exist("edrnpf").await?,
        false,
        "edrnpf should not exist"
    );
    ret_eq!(
        bs.service_exist("trantect-edr-registry").await?,
        false,
        "reg should not exist"
    );
    ret_eq!(
        bs.service_exist("trantect-edr-file-system").await?,
        false,
        "edr-file should not exist"
    );
    Ok(true)
}

#[test]
fn test_install() {
    let ret = task::block_on(async_test_install()).unwrap();
    println!("{:?}", ret);
    assert!(ret);
}
