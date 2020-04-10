pub mod args;
pub mod read;
pub mod stats;
pub mod write;

// 16kb max
const CHUNK_SIZE: usize = 16 * 1024;
