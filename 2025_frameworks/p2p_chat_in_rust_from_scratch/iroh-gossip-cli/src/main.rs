use std::process;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::signal;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stdin_lines = BufReader::new(io::stdin()).lines();

    println!(
        "==> Type something, or press Ctrl+C to exit. Auto-exits after 60 seconds of inactivity."
    );

    loop {
        tokio::select! {
            // If the user types within 60 seconds, read and print
            maybe_line = time::timeout(Duration::from_secs(60), stdin_lines.next_line()) => {
                match maybe_line {
                    Ok(Ok(Some(line))) => {
                        println!(">>> You typed: {}", line);
                    }
                    Ok(Ok(None)) => {
                        println!("<== End of input (EOF). Exiting.");
                        break;
                    }
                    Ok(Err(e)) => {
                        eprintln!("!!! Error reading input: {}", e);
                        break;
                    }
                    Err(_) => {
                        println!("!!! No input received in 60 seconds. Exiting.");
                        break;
                    }
                }
            }

            // Ctrl+C support
            _ = signal::ctrl_c() => {
                println!("\n!!! Received Ctrl+C. Exiting.");
                break;
            }
        }
    }

    println!("<== Goodbye!");
    io::stdout().flush().await?;
    process::exit(0);
    // Ok(())
}
