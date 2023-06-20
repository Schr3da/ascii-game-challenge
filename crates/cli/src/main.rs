mod export;
mod handlers;
mod init;
mod managers;
mod shutdown;
mod traits;
mod utils;
mod views;
mod popup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let terminal = init::terminal().await?;
    shutdown::terminal(terminal)?;
    std::process::exit(0);
}
