use anyhow::Result;
use pimalaya::time::pomodoro::{Server, ServerBind};

pub fn start(binders: Vec<Box<dyn ServerBind>>) -> Result<()> {
    Server::new(binders).bind()?;
    Ok(())
}
