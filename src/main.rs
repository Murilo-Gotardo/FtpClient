use std::io;
use std::net::{Shutdown, TcpStream};
use std::path::Path;

mod commands;

fn main() -> io::Result<()> {
    let mut connection = TcpStream::connect("192.168.0.4:11000")
        .expect("Falha ao conectar com o servidor");

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

        input.clear()
    }

    connection.shutdown(Shutdown::Both)?;
    Ok(())
}

