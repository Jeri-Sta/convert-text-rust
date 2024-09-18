use crate::services::file_services::{read_files, write_file};

pub(crate) fn format_to_java() {
    println!("Lendo arquivo e formatando para Java...");

    let input = read_files("input.txt");

    let mut output = input.replace("\\n", "\r\n");
    output = output.replace("\\t", "\t");

    write_file("output.txt", &output);
}

pub(crate) fn format_to_json() {
    println!("Lendo arquivo e formatando para JSON...");

    let input = read_files("input.txt");

    let mut output = input.replace("\r\n", "\\n");
    output = output.replace("\t", "\\t");

    write_file("output.txt", &output);
}