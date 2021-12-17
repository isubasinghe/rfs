mod cli;

mod fs;
fn main() {
    let opts = cli::parse_opts();
    println!("Hello, world!");
}
