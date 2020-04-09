use std::io::{self, Read, Write};

// 16kb max
const CHUNK_SIZE:usize = 16 * 1024;

fn main() {
    let mut buffer = [0; CHUNK_SIZE];

    // Opening the standard input of the current process. In this case is the terminal.
    // The standard input is the keyboard.
    // The read function pulls bytes from the stdin source into the specified buffer and returns
    // how many bytes were read
    // In this case the buffer is an array
    let num_read = io::stdin().read( &mut buffer).unwrap();

    eprintln!("num_read: {}", num_read);

    io::stdout().write_all(&buffer[..num_read]).unwrap();
}
