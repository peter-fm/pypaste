use atty::Stream;
use pypaste::process_string;
use std::io::{self, Read};

fn main() {
    if atty::is(Stream::Stdin) {
        println!("No input piped in. Exiting.");
        return;
    }

    let mut buffer = String::new();

    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    let processed_data = process_string(buffer.as_str());

    println!("{}", processed_data);
}
