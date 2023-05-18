mod export;
mod init;
mod handlers;
mod managers;
mod shutdown;
mod traits;
mod utils;
mod views;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let terminal = init::terminal().await?;
    shutdown::terminal(terminal)
}
