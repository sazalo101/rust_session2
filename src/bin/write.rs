use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = "Stop acting fuck";

    // Handle potential error from fs::write
    fs::write("quote.txt", contents)?;

    println!("{} patched", contents);

    Ok(())
}
