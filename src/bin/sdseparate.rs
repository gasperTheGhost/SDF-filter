use clap::{load_yaml, App};
use sdf::sdfrecord::SDFRecord;
use std::collections::HashMap;

fn main() {
    // Collect help information and arguments
    let yaml = load_yaml!("help/sdseparate.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let idfield = matches.value_of("idfield").unwrap();
    let zipped = matches.is_present("zipped");

    // Read the file into Vec<Vec<String>>
    let file = sdf::prepare_file_for_SDF(input, zipped);

    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    for block in file {
        // Turn Vec<String> into SDFRecord
        let mut record = SDFRecord::new();
        record.readRec(block);

        // Make a HashMap of file lines and their names
        let id = record.getData(idfield);
        if files.contains_key(&id) {
            record.lines.push("$$$$".to_string());
            files.get_mut(&id).unwrap().extend(record.lines);
        } else {
            record.lines.push("$$$$".to_string());
            files.insert(id, record.lines);
        }
    }

    // Write 
    for (id, lines) in files {
        sdf::write_to_file(&lines.join("\n"), &(output.to_string() + "/" + &id + ".sdf"));
    }
}