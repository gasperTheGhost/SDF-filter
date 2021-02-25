extern crate clap;
use clap::{load_yaml, App};

// Returns number of SDF records in file or stdin

fn main() {
    // Collect help information and arguments
    let yaml = load_yaml!("help/sdcount.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = match matches.is_present("input") {
        true => matches.value_of("input").unwrap(),
        false => "-"
    };

    // Read input file to string
    let file = sdf::read_to_string(input, matches.is_present("zipped"));
    // Print number of SDRecords in string
    println!("{}", file.matches("$$$$").count().to_string())

}