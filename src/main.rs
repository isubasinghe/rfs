mod cli;
mod networking;

use tokio::*;

mod fs;

#[tokio::main]
async fn main() {
    let opts = cli::parse_opts();
    println!("Hello, world!");
}
