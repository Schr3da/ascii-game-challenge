mod export;
mod init;
mod state;
mod views;

use init::init_terminal;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_terminal()
}
