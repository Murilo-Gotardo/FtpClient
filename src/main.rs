use std::any::Any;
use std::io;
use std::net::{IpAddr, Shutdown, TcpStream};
use std::path::Path;

mod commands;

fn main() -> io::Result<()> {
    let mut ip_input = String::new();
    println!("Digite o ip e porta do servidor");

    loop {
        io::stdin().read_line(&mut ip_input).expect("falha ao ler diretorio");

        if validate_ip_and_port(ip_input.trim().to_owned().as_str()) { break }

        ip_input = String::new();
        println!("IP invalido, tente novamente");
    }

    let mut connection = TcpStream::connect(ip_input.trim())
        .expect("Falha ao conectar com o servidor");

    let mut op: i8 = 1;
    let mut input = String::new();
    while op != 0 {
        println!("Qual ação voce deseja realizar?\n
        1 - list\n
        2 - put\n
        3 - get\n
        0 - sair");
        
        loop {
            io::stdin().read_line(&mut input).expect("falha ao ler diretorio");
            
            op = match input.trim().parse() {
                Ok(num) => {
                    num
                } Err(..) => {
                    println!("Numero invalido, digite novamente");
                    5
                }
            };
            
            let value = &op as &dyn Any;
            
            if let Some(_) = value.downcast_ref::<i8>() {  
                break
            }
        }
        
        match op {
            0 => break,
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
            _ => (),
        }

        input.clear()
    }

    connection.shutdown(Shutdown::Both)?;
    Ok(())
}

fn is_valid_ip(ip: &str) -> bool {
    match ip.parse::<IpAddr>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_valid_port(port: u16) -> bool {
    (0..=65535).contains(&port)
}

fn validate_ip_and_port(ip_and_port: &str) -> bool {
    if let Some((ip, port_str)) = ip_and_port.split_once(':') {
        if !is_valid_ip(ip) {
            return false;
        }

        if let Ok(port) = port_str.parse::<u16>() {
            return is_valid_port(port);
        }
    }

    false
}

