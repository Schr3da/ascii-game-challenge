mod export;
mod init;
mod state;
mod views;
mod utils;

use init::init_terminal;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_terminal()
}
