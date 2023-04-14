use anyhow::Result;
use pimalaya::time::pomodoro::Server;

pub fn start(server: Server) -> Result<()> {
    server.bind()?;
    Ok(())
}
