use std::{io, thread};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::{Arc, Mutex};

mod commands;

fn main() {
    let connection = Arc::new(Mutex::new(TcpStream::connect("26.101.125.56:11000")
        .expect("Falha ao conectar com o servidor")));
    
    let mut op: i8 = 1;
    let mut input = String::new();
    while op != 0 {
        println!("Qual ação voce deseja realizar?\n
        1 - list\n
        2 - put\n
        3 - get\n
        0 - sair");
        io::stdin().read_line(&mut input).expect("falha ao ler diretorio");
        op = input.trim().parse().expect("Por favor, insira um número válido");

        let connection_clone = Arc::clone(&connection);

        thread::spawn(move || {
            let mut connection = connection_clone.lock().unwrap();

            match op {
                1 => commands::list(&mut connection),
                2 => {
                    println!("Forneça o diretório do arquivo");
                    let mut path_input = String::new();
                    io::stdin().read_line(&mut path_input).expect("Falha ao ler diretório");
                    let file_name = Path::new(path_input.trim()).file_name().unwrap().to_str().unwrap();
                    commands::put(&mut connection, path_input.trim(), file_name);
                },
                3 => {
                    println!("Forneça o nome do arquivo (com extensão)");
                    let mut file_name_input = String::new();
                    io::stdin().read_line(&mut file_name_input).expect("Falha ao ler diretório");
                    commands::get(&mut connection, file_name_input.trim());
                },
                _ => println!("Valor inválido"),
            }
        });
        
        input.clear()
    }

    thread::sleep(std::time::Duration::from_secs(1));
}

