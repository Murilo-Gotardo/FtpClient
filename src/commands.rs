use std::fs;
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

    send_json_to_server(connection, data);
    let json = receive_json_from_server(connection);

    println!("{}", json)
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

                send_json_to_server(connection, data);
                send_file_to_server(connection, file_path);

                let json = receive_json_from_server(connection);

                println!("{}", json)
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

    send_json_to_server(connection, data);
    let json = receive_json_from_server(connection);
    let file_bytes = receive_file_from_server(connection);

    let new_file_path = format!("{}{}{}", FILES.to_owned(), "/", file_name);
    let output_path = Path::new(&new_file_path);
    fs::write(output_path, file_bytes).expect("falha");

    println!("{}", json)
}

fn send_json_to_server(connection: &mut TcpStream, data: Value) {
    let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");
    let metadata = json_string.len() as u64;
    let bytes = json_string.as_bytes();

    connection.write_all(&metadata.to_le_bytes()).expect("TODO: panic message");
    connection.write_all(bytes).unwrap();
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
    connection.read_exact(&mut buffer).expect("");
    let json_size = u64::from_le_bytes(buffer);

    let mut json_buffer = vec![0; json_size as usize];
    connection.read_exact(&mut json_buffer).expect("");
    let json = String::from_utf8_lossy(&json_buffer[..]);

    return json.to_string();
}

fn receive_file_from_server(connection: &mut TcpStream) -> Vec<u8> {
    let mut buffer = [0; 8];
    connection.read_exact(&mut buffer).expect("");
    let file_size = u64::from_le_bytes(buffer);

    let mut file_buffer = vec![0; file_size as usize];
    connection.read_exact(&mut file_buffer).expect("");

    return file_buffer;
}