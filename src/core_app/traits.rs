use std::io::{Read, Write};

pub trait Compressor {
    fn new() -> Self;
    fn compress<R: Read, W: Write>(&mut self, input: R, output: W) -> std::io::Result<()>;
    fn decompress<R: Read, W: Write>(&mut self, input: R, output: W) -> std::io::Result<()>;
}
