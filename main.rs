use std::fs::File;
use std::io::{Read, Write};
use std::env;
use tdl_rust_c2_2023_rs::lzw::lzw::LzwCompressor;

fn agregar_extension(nombre: &str, extension: &str) -> String {
    let mut parts: Vec<&str> = nombre.split('.').collect();
    if parts.len() > 1 {
        parts.pop();
    }
    parts.push(extension);
    parts.join(".")
}

fn comprimir(input_file_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_file_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;
    let output_file_path = agregar_extension(input_file_path, "lzw");
    let mut output_file = File::create(&output_file_path)?;
    let mut output_buffer = Vec::new();  
    let mut compressor = LzwCompressor::new();
    compressor.compress(&input_data[..], &mut output_buffer)?;
    output_file.write_all(&output_buffer)?;
    println!("Archivo comprimido guardado como: {}", output_file_path);
    Ok(())
}

fn descomprimir(input_file_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_file_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;
    let output_file_path = agregar_extension(input_file_path, "txt");
    let mut output_file = File::create(&output_file_path)?;
    let mut output_buffer = Vec::new();
    let mut compressor = LzwCompressor::new();
    compressor.decompress(&input_data[..], &mut output_buffer)?;
    output_file.write_all(&output_buffer)?;
    println!("Archivo descomprimido guardado como: {}", output_file_path);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Uso: cargo run <comprimir/descomprimir> <archivo_entrada>");
        return Ok(());
    }
    let accion = &args[1];
    let archivo_entrada = &args[2];
    match accion.as_str() {
        "comprimir" => comprimir(archivo_entrada)?,
        "descomprimir" => descomprimir(archivo_entrada)?,
        _ => {
            println!("Acción no reconocida. Use 'comprimir' o 'descomprimir'.");
            return Ok(());
        }
    };
    Ok(())
}