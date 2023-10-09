use anyhow::Result;
use time::Server;

pub fn start(server: Server) -> Result<()> {
    server.bind()?;
    Ok(())
}
