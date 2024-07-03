use std::io;
use std::net::TcpStream;
use std::path::Path;
mod commands;

fn main() {
    let mut connection = TcpStream::connect("127.0.0.1:11000").expect("TODO: panic message");

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
            0 => return,
            1 => commands::list(&mut connection),
            2 => {
                println!("Forneça o diretório do arquivo");
                let mut path_input = String::new();
                io::stdin().read_line(&mut path_input).expect("falha ao ler diretório");
                let file_name = Path::new(path_input.as_str()).file_name().unwrap().to_str().unwrap();
                commands::put(&mut connection, path_input.as_str(), file_name)
            },
            3 => {
                println!("Forneça o nome do arquivo (com extensão)");
                let mut file_name_input = String::new();
                io::stdin().read_line(&mut file_name_input).expect("falha ao ler diretório");
                commands::get(&mut connection, file_name_input.as_str())
            },
            _ => println!("Valor inválido")
        }
    }
}

