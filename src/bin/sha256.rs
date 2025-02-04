use ring::digest::{Context, SHA256};

fn main() {
    let data = b"Hello, world"; // Correct byte slice

    let mut context = Context::new(&SHA256);
    context.update(data);
    let hash = context.finish(); // Get the final hash

    println!("SHA256 Hash: {:?}", hash.as_ref()); // Correctly print the hash
}
