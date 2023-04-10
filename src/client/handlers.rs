use anyhow::Result;
use pimalaya::time::pomodoro::Client;

pub fn start(client: &dyn Client) -> Result<()> {
    client.start()?;
    Ok(())
}

pub fn get(client: &dyn Client) -> Result<()> {
    let timer = client.get()?;
    println!("{}", timer.value);
    Ok(())
}

pub fn pause(client: &dyn Client) -> Result<()> {
    client.pause()?;
    Ok(())
}

pub fn resume(client: &dyn Client) -> Result<()> {
    client.resume()?;
    Ok(())
}

pub fn stop(client: &dyn Client) -> Result<()> {
    client.stop()?;
    Ok(())
}
