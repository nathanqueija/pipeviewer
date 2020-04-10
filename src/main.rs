use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// 16kb max
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        // Opening the standard input of the current process. In this case is the terminal.
        // The standard input is the keyboard.
        // The read function pulls bytes from the stdin source into the specified buffer and returns
        // how many bytes were read
        // In this case the buffer is an array
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        if !silent {
            eprintln!("\r{}", total_bytes);
        }

        // If you use the ? syntax it means that if this operations returns an Error
        // it will be returned from this function
        // io::stdout().write_all(&buffer[..num_read])?

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    if !silent {
        eprintln!("\r{}", total_bytes);
    }

    Ok(())
}
