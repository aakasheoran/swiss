mod cat;

use std::env;
use std::io;

const VERSION: &str = "v1.0.0";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "cat" {
        return cat::display_file_content(&args);
    }

    if args.iter().any(|arg| arg == "-v" || arg == "--version") {
        println!("swiss version {}", VERSION);
        return Ok(());
    }

    Ok(())
}
