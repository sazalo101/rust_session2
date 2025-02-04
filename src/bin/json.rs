use std::fs::File;
use std::io::{self, Write};
use serde_json::json;

fn main() -> io::Result<()> {
    let data = json!({
        "message": "Hello, Rust!",
        "status": "success"
    });

    let mut file = File::create("output.json")?;
    file.write_all(data.to_string().as_bytes())?;

    println!("JSON written successfully.");
    Ok(())
}
