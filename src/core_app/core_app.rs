use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    thread::{self, ThreadId},
};

use crate::{lzw::lzw::LzwCompressor, utils::files::add_extension};

#[derive(Default)]
pub struct CompressorAppState {
    pub processes: RefCell<HashMap<String, ThreadId>>,
}

impl CompressorAppState {
    fn add_process(&self, name: &str, pid: ThreadId) {
        let mut processes = self.processes.borrow_mut();
        let thread_id = processes.entry(name.to_string()).or_insert(pid);
        *thread_id = pid.clone();
    }

    pub fn print_all_process(&self) {
        self.processes.borrow().iter().for_each(|(name, pid)| {
            println!("Process: {}: {:?}", name, pid);
        });
    }

    pub fn compress(&self, input_file_path: &str) -> std::io::Result<()> {
        let mut input_file = File::open(input_file_path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;
        let output_file_path = add_extension(input_file_path, "lzw");
        let mut output_file = File::create(&output_file_path)?;
        let mut output_buffer = Vec::new();

        let job = thread::spawn(move || {
            let mut compressor = LzwCompressor::new();
            compressor
                .compress(&input_data[..], &mut output_buffer)
                .unwrap();
            output_file.write_all(&output_buffer).unwrap();
            println!("Archivo comprimido guardado como: {}", output_file_path);
        });

        let pid = job.thread().id().clone();
        self.add_process(input_file_path, pid);
        Ok(())
    }

    pub fn decompress(&self, input_file_path: &str) -> std::io::Result<()> {
        let mut input_file = File::open(input_file_path)?;
        let mut input_data = Vec::new();
        input_file.read_to_end(&mut input_data)?;
        let output_file_path = add_extension(input_file_path, "txt");
        let mut output_file = File::create(&output_file_path)?;
        let mut output_buffer = Vec::new();
        let job = thread::spawn(move || {
            let mut compressor = LzwCompressor::new();
            compressor
                .decompress(&input_data[..], &mut output_buffer)
                .unwrap();
            output_file.write_all(&output_buffer).unwrap();
            println!("Archivo descomprimido guardado como: {}", output_file_path);
        });

        let pid = job.thread().id().clone();
        self.add_process(input_file_path, pid);
        Ok(())
    }
}
