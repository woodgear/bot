use bot_client_rust::{ret_eq,prelude::*};
use async_std::task;
use std::path::PathBuf;

async fn async_test_install() -> Result<bool, failure::Error> {
    let url = "ws://127.0.0.1:12345";
    let bot = BotClient::new(url);
    let fs = bot.fs();
    let bs = bot.shell();

    let r_root = fs.assign_dir().await?;
    let runtime = PathBuf::from(&r_root).join("runtime");
    fs.copy_dir(r#"C:\Users\developer\Desktop\runtime"#,&runtime).await?;

    ret_eq!(bs.service_exist("edrnpf").await?,false);
    ret_eq!(bs.service_exist("registry_driver").await?,false);
    ret_eq!(bs.service_exist("trantect-edr-file-system").await?,false);
    // tail  
    // bs.exec(&format!(r#"{}\edr_agent.exe install"#,root_path)).await?;
    
    // tail.should_see("",1.secs())
    //     .shoud_not_seed("error",4.ses()).await;

    // ret_eq!(bs.service_exist("edrnpf").await?,true);
    // ret_eq!(bs.service_exist("registry_driver").await?,true);
    // ret_eq!(bs.service_exist("trantect-edr-file-system").await?,true);

    // ret_eq!(bs.service_running("edrnpf").await?,true);
    // ret_eq!(bs.service_running("registry_driver").await?,true);
    // ret_eq!(bs.service_running("trantect-edr-file-system").await?,true);

    Ok(true)
}

    #[test]
    fn test_install() {
        let ret = task::block_on(async_test_install()).unwrap();
        assert!(ret);
    }