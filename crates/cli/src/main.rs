mod entry;
mod export;
mod handlers;
mod managers;
mod utils;

use entry::Entry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut entry = Entry::default();
    entry.init().await?;
    drop(entry);

    std::process::exit(0);
}
