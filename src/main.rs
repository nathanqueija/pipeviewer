use std::io::{self, Read, Write};
use std::env;

// 16kb max
const CHUNK_SIZE:usize = 16 * 1024;

fn main() {
    let silent = env::var("PV_SILENT").unwrap_or(String::new()).len() > 0;

    let mut total_bytes = 0;

    loop{
        let mut buffer = [0; CHUNK_SIZE];

        // Opening the standard input of the current process. In this case is the terminal.
        // The standard input is the keyboard.
        // The read function pulls bytes from the stdin source into the specified buffer and returns
        // how many bytes were read
        // In this case the buffer is an array
        let num_read = match io::stdin().read( &mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    if !silent {
        eprintln!("Total bytes read: {}", total_bytes);
    }

}
