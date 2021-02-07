use std::{
    fs::File,
    path::Path,
    io::{self, Write, BufReader, BufRead}
};
extern crate clap;
use clap::{load_yaml, App};

// Converts between CRLF and LF line separators

fn main() {
    let yaml = load_yaml!("help/lineconv.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = match matches.is_present("input") {
        true => matches.value_of("input").unwrap(),
        false => "-"
    };

    if matches.is_present("bytes") {
        let mut reader: Box<dyn BufRead> = match input {
            "-" => Box::new(BufReader::new(io::stdin())),
            _ => Box::new(BufReader::new(File::open(Path::new(input)).expect("No such file")))
        };
        let mut buf: Vec<u8> = Vec::new();
        let mut file: Vec<u8> = Vec::new();
        while let Ok(_) = reader.read_until(b'\n', &mut buf) {
            if buf.is_empty() {
                break;
            }
            if buf.last().unwrap().to_owned() == b'\r' {
                &buf.pop();
            }
            match matches.is_present("crlf") {
                true => {&buf.push(b'\r'); &buf.push(b'\n')},
                false => &buf.push(b'\n')
            };
            file.extend(&buf);
            buf.clear();
        }
        if matches.is_present("output") {
            sdf::write_bytes_to_file(file, matches.value_of("output").unwrap());
        } else {    
            io::stdout().write_all(&file).expect("Error writing to stdout");
        };
    } else {
        let separator = match matches.is_present("crlf") {
            true => "\r\n",
            false => "\n"
        };

        let file = sdf::lines_from_file(input);

        if matches.is_present("output") {
            sdf::write_to_file(&file.join(separator), matches.value_of("output").unwrap());
        } else {    
            io::stdout().write_all(file.join(separator).as_bytes()).expect("Error writing to stdout");
        };
    }
}