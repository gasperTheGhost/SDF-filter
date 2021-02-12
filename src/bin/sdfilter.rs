use std::{
    io::{self, Write},
    fs::metadata,
    process,
    time::Instant
};
use clap::{load_yaml, App};
use indicatif::{HumanDuration, ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use sdf::sdfrecord::SDFRecord;

fn main() {
    // Collect help information and arguments
    let yaml = load_yaml!("help/sdfilter.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("input");
    let output = matches.value_of("output").unwrap();
    let field = matches.value_of("field").unwrap();
    let value: f64 = matches.value_of("value").unwrap().parse().expect("Value is not valid float!");
    let operand = matches.value_of("operand").unwrap();
    let filetypes: Vec<&str> = vec!["test"]; //matches.values_of("filetypes").unwrap().collect();
    
    // Iterate over input files 
    let files: Vec<String>;
    if let Some(path) = input {
        if path == "-"{ // If input is "-" then read from stdin
            files = vec![(&path).to_string()];
        } else if metadata(&path).unwrap().is_dir() { // Check if path points to dir
            files = sdf::getFiles(&path, filetypes.clone(), (&matches.is_present("recursive")).to_owned());
        } else { // Check if path points to a file
            files = vec![(&path).to_string()];
        }

        if output == "-" {
            for file in files { // Use par_iter() for easy parallelization
                // Read file contents to string
                let mut contents = sdf::lines_from_file(&file);
                
                // Iterate over SDRecords
                let mut matching_records: Vec<String> = Vec::new();
                let mut record: SDFRecord = SDFRecord::new();
                while &contents.len() > &0 {
                    // Turn vector into SDFRecord
                    contents = record.readRec(contents);

                    // Get matching records
                    if evaluate(record.copy(), field, value, operand) {
                        matching_records.push(record.lines.join("\n"));
                    }
                }

                // Write vector of extracted data to stdout
                io::stdout().write_all(matching_records.join("\n").as_bytes()).expect("Error writing to stdout");
            }
        } else {
            // Draw a nice progress bar
            let started = Instant::now();
            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta} @ {per_sec})")
                .progress_chars("#>-"));
            
            // Iterate over files in directory (or single specified file)
            println!("Processing files...");

            let _iter: Vec<_> = files.par_iter().progress_with(pb).map(|file| { // Use par_iter() for easy parallelization
                // Read file contents to string
                let mut contents = sdf::lines_from_file(&file);
                
                // Iterate over SDRecords
                let mut matching_records: Vec<String> = Vec::new();
                let mut record: SDFRecord = SDFRecord::new();
                while &contents.len() > &0 {
                    // Turn vector into SDFRecord
                    contents = record.readRec(contents);

                    // Get matching records
                    if evaluate(record.copy(), field, value, operand) {
                        matching_records.push(record.lines.join("\n"));
                    }
                }

                // Set output path
                let out_path: String = match file.trim() {
                    "-" => (output.to_owned() + "/stdin.txt"),
                    _ => (output.to_owned() + "/" + (&file.split("/").collect::<Vec<&str>>()).last().unwrap()),
                };
                // Write vector of extracted data to file
                sdf::write_to_file(&(matching_records.join("\n")), &out_path);
            }).collect();

            println!("Done in {}", HumanDuration(started.elapsed()));
        }

    } else {
        // Exit if input doesn't exist
        process::exit(0x0100);
    }
}

fn evaluate(record: SDFRecord, field: &str, value: f64, operand: &str) -> bool {
    match operand {
        "lt" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() < value {
                return true;
            } else {return false}
        },
        "le" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() <= value {
                return true;
            } else {return false}
        },
        "eq" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() == value {
                return true;
            } else {return false}
        },
        "ne" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() != value {
                return true;
            } else {return false}
        },
        "ge" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() >= value {
                return true;
            } else {return false}
        },
        "gt" => {
            if record.data[&field.to_string()][0].parse::<f64>().unwrap() > value {
                return true;
            } else {return false}
        },
        _ => {
            eprintln!("Unsupported operand!");
            return false;
        }
    }
}