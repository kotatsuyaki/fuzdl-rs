use std::{process::Stdio, time::Duration};

use anyhow::{Context, Result};
use dialoguer::{Confirm, Input, Password};
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::{
    process::{Child, Command},
    task::spawn_blocking,
};

#[macro_use]
mod macros;
mod states;

#[tokio::main]
async fn main() -> Result<()> {
    let mut driver_process = create_driver_process()?;
    let driver = create_driver().await?;

    // The `run` function is just for the error-catching boundary
    if let Err(e) = run(&driver).await {
        warn!("Encountered error: {}", e);
    }

    spawn_blocking(|| Confirm::new().with_prompt("Pause").interact()).await??;

    info!("Quitting driver");
    driver.quit().await?;
    driver_process.kill().await?;

    Ok(())
}

async fn run(driver: &WebDriver) -> Result<()> {
    let (email, password) = prompt_credentials().await?;

    let signin_state = states::Signin::new(&driver).await?;
    info!("Navigated to signin page");

    signin_state.signin(&driver, email, password).await?;
    info!("Successfully signed in");

    Ok(())
}

fn create_driver_process() -> Result<Child> {
    Command::new("chromedriver")
        .arg("--port=4444")
        // Prevent IO from the child process messing up our IO
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to spawn chromedriver")
}

async fn create_driver() -> Result<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    caps.add_chrome_arg("--no-sandbox")?;

    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .context("Failed to create WebDriver")?;

    // Enable waiting period before find timeouts
    driver
        .set_implicit_wait_timeout(Duration::from_secs(10))
        .await
        .context("Failed to set timeout for WebDriver")?;

    // The width 960 (which is small enough)
    // prevents Fuz from showing two manga pages at once
    driver
        .set_window_rect(0, 0, 960, 1080)
        .await
        .context("Failed to set window rect")?;

    Ok(driver)
}

async fn prompt_credentials() -> Result<(String, String)> {
    let email: String = spawn_blocking(|| Input::new().with_prompt("Email").interact())
        .await?
        .context("Failed to read email")?;
    let password = spawn_blocking(|| Password::new().with_prompt("Password").interact())
        .await?
        .context("Failed to read password")?;
    Ok((email, password))
}