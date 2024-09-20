mod services;

use std::io;
use std::io::{Write};

use crate::services::file_services::create_files;
use crate::services::format_service::{format_to_json, format_to_java};

fn main() {
    create_files();
    println!("Olá!");

    loop {
        println!("Qual opção de formatação você deseja realizar? \n\
        1 - Formatar para JSON \n\
        2 - Formatar para código Java \n\
        0 - Fechar programa");

        let mut input = String::new();

        if let Err(_) = io::stdout().flush() {
            println!("Falha ao limpar o buffer de saída")
        }

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Falha ao ler a entrada")
        }

        let input = input.trim();

        match input {
            "1" => {
                println!("Você escolheu a opção 1 - Formatar para JSON");
                format_to_json();
            },
            "2" => {
                println!("Você escolheu a opção 2 - Formatar para código Java");
                format_to_java();
            },
            "0" => {
                println!("Você escolheu a opção 0 - Fechar programa");
                println!("Até mais!");
                break;
            },
            _ => println!("Opção inválida!"),
        }
    }
}
