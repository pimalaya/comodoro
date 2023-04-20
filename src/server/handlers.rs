use anyhow::Result;
use pimalaya_pomodoro::Server;

pub fn start(server: Server) -> Result<()> {
    server.bind()?;
    Ok(())
}
