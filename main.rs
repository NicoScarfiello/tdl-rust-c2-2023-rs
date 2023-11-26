use std::fs::{self};
use std::io::{self, Write};
use tdl_rust_c2_2023_rs::core_app::core_app::CompressorAppState;

thread_local! {
    static APP_STATE: CompressorAppState = CompressorAppState::default()
}

fn compress(input_file_path: &str) -> std::io::Result<()> {
    APP_STATE.with(|app_state| app_state.compress(input_file_path))
}

fn decompress(input_file_path: &str) -> std::io::Result<()> {
    APP_STATE.with(|app_state| app_state.decompress(input_file_path))
}

pub fn ask_for_file() -> Result<String, ()> {
    loop {
        println!("Ingrese el nombre del archivo");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Error al leer el nombre del archivo");
        file_name = file_name.trim().to_string();
        match fs::metadata(&file_name) {
            Ok(_) => {
                return Ok(file_name);
            }
            Err(_) => {
                println!("El archivo no existe");
            }
        };
    }
}

fn print_jobs() {
    APP_STATE.with(|app_state| app_state.print_all_process())
}

#[allow(dead_code)]
fn create_super_big_file() {
    let mut file = fs::File::create("super_big_file.txt").unwrap();
    for _ in 0..1000000 {
        file.write_all(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. Sed sit amet ipsum mauris. Maecenas congue ligula ac quam viverra nec consectetur ante hendrerit. Donec et mollis dolor. Praesent et diam eget libero egestas mattis sit amet vitae augue.").unwrap();
    }
}

fn main() -> std::io::Result<()> {
    // create_super_big_file();
    let mut action: String = String::new();
    let mut stop: bool = false;
    while !stop {
        println!("Que desea hacer? (comprimir/descomprimir/salir)");
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        println!("Usted eligio: {}", action);
        action = action.trim().to_string();
        match action.as_str() {
            "comprimir" => {
                let file_name = ask_for_file();
                match file_name {
                    Ok(file_name) => {
                        let _ = &action.clear();
                        compress(&file_name)?
                    }
                    Err(_) => {
                        let _ = &action.clear();
                        println!("Error al leer el nombre del archivo")
                    }
                }
            }
            "descomprimir" => {
                let file_name = ask_for_file();
                match file_name {
                    Ok(file_name) => {
                        let _ = &action.clear();
                        decompress(&file_name)?
                    }
                    Err(_) => {
                        let _ = &action.clear();
                        println!("Error al leer el nombre del archivo")
                    }
                }
            }
            "salir" => stop = true,
            _ => {
                println!("Opcion no reconocida");
                let _ = &action.clear();
            }
        }
        print_jobs();
    }
    Ok(())
}
