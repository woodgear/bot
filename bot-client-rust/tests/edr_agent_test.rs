use async_std::stream::StreamExt;
use async_std::task;
use bot::protocol::ExecConfig;
use bot_client_rust::{prelude::*, ret_eq};
use std::path::PathBuf;
use std::time::Duration;
async fn init_package(pipeline_id: String) {}

async fn service_should_exists(bs: &mut BotShell, services: Vec<&str>) -> Result<bool, failure::Error> {
    for s in services {
        ret_eq!(
            bs.service_exist(s).await?,
            true,
            &format!("service {} should exist", s)
        );
    }
    Ok(true)
}

async fn service_should_not_exists(
    bs: &mut BotShell,
    services: Vec<&str>,
) -> Result<bool, failure::Error> {
    for s in services {
        ret_eq!(
            bs.service_exist(s).await?,
            false,
            &format!("service {} should not exist", s)
        );
    }
    Ok(true)
}

async fn async_test_install(url:&str) -> Result<bool, failure::Error> {
    let bot = BotClient::new(&format!("ws://{}:12345",url));
    let fs = bot.fs();
    let mut bs = bot.shell();

    let r_root = fs.assign_dir().await?;
    let runtime = PathBuf::from(&r_root).join("runtime");
    // let runtime =
    //     PathBuf::from(r#"C:\Users\developer\work\lab\bot\bot-server-rust\data\QhHFoSt\runtime"#);
    fs.copy_dir(r#"C:\Users\developer\Desktop\runtime"#, &runtime)
        .await;
    let runtime_str = runtime.to_string_lossy().to_string();
    //TODO tail error invalid file name
    let log_path = runtime.join(r#"log\3rt\edr-agent.log"#);
    let log_path_str = log_path.to_string_lossy().to_string();
    println!("{:?}", log_path_str);

    ret_eq!(
        service_should_not_exists(
            &mut bs,
            vec!["edrnpf","trantect-edr-registry", "registry_driver", "trantect-edr-file-system"]
        )
        .await?,
        true
    );

    let cmd = format!(r#"{}\edr_agent.exe install"#, runtime_str);

    let mut tail = bot.tail(&log_path_str).await?;
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

    ret_eq!(res.0?, true, "should match log");
    
    //旧的服务名应当被删掉
    ret_eq!(
        service_should_not_exists(
            &mut bs,
            vec!["registry_driver"]
        )
        .await?,
        true
    );

    ret_eq!(
        service_should_exists(
            &mut bs,
            vec!["edrnpf", "trantect-edr-registry", "trantect-edr-file-system"]
        )
        .await?,
        true
    );

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

    let res = bs
        .exec(ExecConfig {
            cmd,
            cwd: runtime_str.to_string(),
        })
        .await;
    println!("uninstall over res {:?}", res);

    ret_eq!(
        service_should_not_exists(
            &mut bs,
            vec!["edrnpf", "registry_driver", "trantect-edr-file-system"]
        )
        .await?,
        true
    );

    Ok(true)
}

#[test]
fn test_install() {
    let urls = vec!["127.0.0.1",];
    let all =  futures::future::join_all(urls.iter().map(|url|async_test_install(url)));
    
    let ret = task::block_on(all);
    for (index,test_result) in ret.iter().enumerate() {
        if let Ok(true) = test_result {
            println!("{:?} success",urls[index]);
        } else {
            panic!(format!("{} fail {:?}",urls[index],test_result));
        }
    }
}
