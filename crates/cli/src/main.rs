mod export;
mod init;
mod state;
mod views;
mod utils;
mod input;
mod shutdown;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let terminal = init::terminal().await?;
    shutdown::terminal(terminal)
}
