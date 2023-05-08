mod export;
mod init;
mod macros;
mod managers;
mod shutdown;
mod state;
mod traits;
mod utils;
mod views;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let terminal = init::terminal().await?;
    shutdown::terminal(terminal)
}
