use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read(infile: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];

    // Opening the standard input of the current process. In this case is the terminal.
    // The standard input is the keyboard.
    // The read function pulls bytes from the stdin source into the specified buffer and returns
    // how many bytes were read
    // In this case the buffer is an array

    // If you use the ? syntax it means that if this operations returns an Error
    // it will be returned from this function
    // io::stdout().write_all(&buffer[..num_read])?
    let num_read = reader.read(&mut buffer)?;
    Ok(Vec::from(&buffer[..num_read]))
}
