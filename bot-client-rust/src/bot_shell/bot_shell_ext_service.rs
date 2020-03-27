use crate::bot_shell::BotShell;

impl BotShell {
    pub async fn service_exist(&self, name: &str) -> Result<bool, failure::Error> {
        let res = self.exec(&format!("sc qc {}", name)).await?;
        Ok(res.contains(name))
    }
}
