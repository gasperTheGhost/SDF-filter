use std::io::{self, Write};
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

    let file = sdf::lines_from_file(input);

    let separator = match matches.is_present("crlf") {
        true => "\r\n",
        false => "\n"
    };

    if matches.is_present("output") {
        sdf::write_to_file(&file.join(separator), matches.value_of("output").unwrap());
    } else {    
        io::stdout().write_all(file.join(separator).as_bytes()).expect("Error writing to stdout");
    };
}