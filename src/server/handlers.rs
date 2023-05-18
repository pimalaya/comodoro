use anyhow::Result;
use pimalaya_time::Server;

pub fn start(server: Server) -> Result<()> {
    server.bind()?;
    Ok(())
}
