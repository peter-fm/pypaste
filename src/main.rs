use atty::Stream;
use clap::{App, Arg};
use copypasta::{ClipboardContext, ClipboardProvider};
use pypaste::process_string;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    let matches = App::new("pypaste")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Correctly formats python code for sending to a REPL running on tmux (supports chunking and delays required for macos)")
        .arg(
            Arg::with_name("target")
                .short('t')
                .long("target")
                .takes_value(true)
                .help("The tmux target pane, or `pipe` for stdout (default `pipe`)"),
        )
        .arg(
            Arg::with_name("buffer-size")
                .short('b')
                .long("buffer-size")
                .takes_value(true)
                .help("The size of each chunk sent to tmux (default 1024)"),
        )
        .arg(
            Arg::with_name("delay")
                .short('d')
                .long("delay")
                .takes_value(true)
                .help("Delay between chunks in milliseconds (default 10)"),
        )
        .arg(
            Arg::with_name("clipboard")
                .short('c')
                .long("clipboard")
                .takes_value(false) // No value needed for boolean flags
                .help("Source from clipboard instead of stdin (default stdin)"),
        )
        .get_matches();

    let target = matches.value_of("target").unwrap_or("pipe");
    let default_chunk_size = if cfg!(target_os = "macos") { 900 } else { 1024 };
    let chunk_size = matches
        .value_of("buffer-size")
        .unwrap_or(&default_chunk_size.to_string())
        .parse::<usize>()
        .unwrap();
    let delay = matches
        .value_of("delay")
        .unwrap_or("10")
        .parse::<u64>()
        .unwrap();
    let use_clipboard = matches.is_present("clipboard"); // Check if the flag is present

    let input = if use_clipboard {
        let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");
        ctx.get_contents().expect("Failed to read from clipboard")
    } else {
        // If input is piped, read from stdin
        if atty::is(Stream::Stdin) {
            println!("No input piped in. Exiting.");
            return;
        }
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    let processed_data = process_string(&input); // Assuming this function returns a String

    if target == "pipe" {
        println!("{}", processed_data);
        return;
    }

    let chunks = processed_data.as_bytes().chunks(chunk_size);

    Command::new("stty")
        .arg("-icanon")
        .output()
        .expect("Failed to disable canonical mode");

    for chunk in chunks {
        let chunk_str =
            String::from_utf8(chunk.to_vec()).expect("Failed to convert bytes to string");

        let mut success = false;
        let mut attempts = 0;

        while !success && attempts < 3 {
            // Retry up to 3 times
            attempts += 1;

            let mut child = Command::new("tmux")
                .args(["load-buffer", "-", ";", "paste-buffer", "-t", target])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to execute tmux command");

            if let Some(stdin) = child.stdin.as_mut() {
                stdin
                    .write_all(chunk_str.as_bytes())
                    .expect("Failed to write to stdin");
            }

            success = child.wait().expect("tmux command failed to run").success();

            if !success {
                eprintln!("Retrying chunk due to failure...");
                thread::sleep(Duration::from_millis(delay * 2)); // Add extra delay on failure
            }
        }

        if !success {
            eprintln!("Failed to send chunk after multiple attempts.");
            break;
        }

        thread::sleep(Duration::from_millis(delay));
    }
    Command::new("stty")
        .arg("icanon")
        .output()
        .expect("Failed to re-enable canonical mode");
}
