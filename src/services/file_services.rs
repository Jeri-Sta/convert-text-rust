use std::fs::{File, OpenOptions};
use std::path::Path;

pub(crate) fn create_files() {
    let paths = [Path::new("input.txt"), Path::new("output.txt")];

    for path in paths {
        if path.exists() {
            continue;
        }
        let display = path.display();
        match File::create(&path) {
            Err(why) => panic!("Não foi possível criar o arquivo {}: {}", display, why),
            Ok(file) => file,
        };
    }
}

pub(crate) fn read_files(file_name: &str) -> String {
    use std::io::Read;

    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Não foi possível abrir o arquivo {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Não foi possível ler o arquivo {}: {}", display, why),
        Ok(_) => s,
    }
}

pub(crate) fn write_file(file_name: &str, content: &str) {
    use std::io::Write;

    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match OpenOptions::new().write(true).truncate(true).open(&path) {
        Err(why) => panic!("Não foi possível abrir o arquivo {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Não foi possível escrever no arquivo {}: {}", display, why),
        Ok(_) => println!("Arquivo {} escrito com sucesso!", display),
    }
}