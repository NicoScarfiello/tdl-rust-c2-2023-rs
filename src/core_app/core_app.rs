use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use crate::{lzw::lzw::LzwCompressor, utils::files::add_extension};

#[derive(Default)]
pub struct CompressorAppState {
    pub processes: RefCell<HashMap<String, u16>>,
}

impl CompressorAppState {
    pub fn add_process(&self, name: &str) {
        let mut processes = self.processes.borrow_mut();
        let counter = processes.entry(name.to_string()).or_insert(0);
        *counter += 1;
    }

    pub fn remove_process(&self, name: &str) {
        let mut processes = self.processes.borrow_mut();
        let counter = processes.entry(name.to_string()).or_insert(0);
        *counter -= 1;
    }

    pub fn compress(&self, input_file_path: &str) -> std::io::Result<()> {
        let mut input_file = File::open(input_file_path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;
        let output_file_path = add_extension(input_file_path, "lzw");
        let mut output_file = File::create(&output_file_path)?;
        let mut output_buffer = Vec::new();
        let mut compressor = LzwCompressor::new();
        compressor.compress(&input_data[..], &mut output_buffer)?;
        output_file.write_all(&output_buffer)?;
        println!("Archivo comprimido guardado como: {}", output_file_path);
        Ok(())
    }

    pub fn decompress(&self, input_file_path: &str) -> std::io::Result<()> {
        let mut input_file = File::open(input_file_path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;
        let output_file_path = add_extension(input_file_path, "txt");
        let mut output_file = File::create(&output_file_path)?;
        let mut output_buffer = Vec::new();
        let mut compressor = LzwCompressor::new();
        compressor.decompress(&input_data[..], &mut output_buffer)?;
        output_file.write_all(&output_buffer)?;
        println!("Archivo descomprimido guardado como: {}", output_file_path);
        Ok(())
    }
}
