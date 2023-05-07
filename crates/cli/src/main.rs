mod components;
mod export;
mod init;
mod input;
mod shutdown;
mod state;
mod utils;
mod views;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let terminal = init::terminal().await?;
    shutdown::terminal(terminal)
}
