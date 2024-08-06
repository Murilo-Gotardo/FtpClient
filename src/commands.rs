use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use serde_json::{json, Value};

const FILES: &str = "src/files";

pub fn list(connection: &mut TcpStream) {
    let data = json!({
        "command": "list",
    });
    
    match send_json_to_server(connection, data) { 
        Ok(..) => {
            let json = receive_json_from_server(connection);

            let value: Value = serde_json::from_str(&*json).expect("");

            if let Some(files_list) = value.get("files_list") {
                if let Some(files) = files_list.as_array() {
                    println!("Arquivos no servidor: ");
                    println!("|-- src");
                    for file in files {
                        if let Some(file_name) = file.as_str() {
                            println!("    |-- {}", file_name);
                        }
                    }
                }
            }
        } Err(e) => {
            println!("Conexao perdida")
        }
    }
}

pub fn put(connection: &mut TcpStream, file_path: &str, file_name: &str) {
    let path = Path::new(file_path.trim());
    if path.exists() && path.is_file() {
        match fs::read(path) {
            Ok(bytes) => {
                let local_hash = sha256::digest(&bytes);
                let data = json!({
                    "file_name": file_name.trim(),
                    "command": "put",
                    "hash" : local_hash
                });

                match send_json_to_server(connection, data) {
                    Ok(_) => {
                        send_file_to_server(connection, file_path);

                        let json = receive_json_from_server(connection);
                        let value: Value = serde_json::from_str(&*json).expect("");

                        if let Some(status) = value.get("status") {
                            if status != "success" {
                                println!("Arquivo com erro (nao inserido)");
                                return;
                            }

                            println!("Arquivo inserido com sucesso")
                        }
                    } Err(_) => println!("Conexao perdida")
                }
            }
            Err(e) => {
                println!("Erro ao ler o arquivo: {}", e);
            }
        }
    } else {
        println!("O caminho fornecido não é válido ou não é um arquivo.");
    }
}

pub fn get(connection: &mut TcpStream, file_name: &str) {
    let data = json!({
        "file_name": file_name.trim(),
        "command": "get",
    });
    
    match send_json_to_server(connection, data) { 
        Ok(_) => {
            let json = receive_json_from_server(connection);
            let file_bytes = receive_file_from_server(connection);

            match fs::create_dir_all(FILES.to_owned()) {
                Ok(_) => {
                    let new_file_path = format!("{}{}{}", FILES.to_owned(), "/", file_name);
                    let output_path = Path::new(&new_file_path);
                    fs::write(output_path, file_bytes).expect("falha");

                    let value: Value = serde_json::from_str(&*json).expect("");

                    if let Some(file) = value.get("file_name") {
                        println!("|-- src");
                        println!("    |-- [+] {}", file.to_string());
                    }

                } Err(_) => println!("nao foi possivel criar o caminho")
            }
        } Err(_) => println!("Conexao perdida")
    }
}

fn send_json_to_server(connection: &mut TcpStream, data: Value) -> io::Result<()>{
    let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");
    let metadata = json_string.len() as u64;
    let bytes = json_string.as_bytes();
    
    match connection.write_all(&metadata.to_le_bytes()) {  
        Ok(..) => match connection.write_all(bytes) { 
            Ok(()) => { Ok(()) }
            Err(e) => {
                Err(e)
            }
        },
        Err(e) => {
            Err(e)
        }
    }
}

fn send_file_to_server(connection: &mut TcpStream, file_path: &str) {
    let path = Path::new(file_path.trim());
    if path.exists() && path.is_file() { 
        match File::open(path) {
            Ok(mut file) => {
                let file_size = file.metadata().unwrap().len();
                let mut file_buffer = vec![0; file_size as usize];
                let bytes_read = file.read(&mut file_buffer).expect("REASON");

                connection.write_all(file_size.to_le_bytes().as_slice()).unwrap();
                connection.write_all(&file_buffer[..bytes_read]).unwrap();
            } Err(e) => {
                println!("Erro ao ler o arquivo: {}", e);
            }
        }
    } else {
        println!("O caminho fornecido não é válido ou não é um arquivo");
    }
}

fn receive_json_from_server(connection: &mut TcpStream) -> String {
    let mut buffer = [0; 8];
    connection.read(&mut buffer).expect("");
    let json_size = u64::from_le_bytes(buffer);

    let mut json_buffer = vec![0; json_size as usize];
    connection.read_exact(&mut json_buffer).expect("");
    let json = String::from_utf8_lossy(&json_buffer[..]);

    return json.to_string();
}

fn receive_file_from_server(connection: &mut TcpStream) -> Vec<u8> {
    let mut buffer = [0; 8];
    connection.read(&mut buffer).expect("");
    let file_size = u64::from_le_bytes(buffer);

    let mut file_buffer = vec![0; file_size as usize];
    connection.read_exact(&mut file_buffer).expect("");

    return file_buffer;
}