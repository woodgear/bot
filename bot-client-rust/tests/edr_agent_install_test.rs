use bot_client_rust::{ret_eq,prelude::*};
use async_std::task;

async fn async_test_install() -> Result<bool, failure::Error> {
    let url = "ws://127.0.0.1:12345";
    let bot = BotClient::new(url);
    let fs = bot.fs();
    let data = fs
        .read_file(r#"C:\Users\developer\work\edr\edr-agent\README.md"#.to_string())
        .await?;
    let msg = String::from_utf8(data)?;

    println!("{}", msg);
    let bs = bot.shell();
    let out = bs.exec("echo status").await?;

    let root_path = r#"C:\Users\developer\Downloads\runtime"#;

    fs.copy_dir(r#"C:\Users\developer\Desktop\runtime"#,root_path).await?;
    
    ret_eq!(bs.service_exist("edrnpf").await?,false);
    ret_eq!(bs.service_exist("edrnpf").await?,false);
    ret_eq!(bs.service_exist("edrnpf").await?,false);
    
    bs.spawn(&format!(r#"{}\edr_agent.exe install"#,root_path)).await?;

    println!("{}", out);
    Ok(true)
}

    #[test]
    fn test_install() {
        let ret = task::block_on(async_test_install()).unwrap();
        assert!(ret);
    }