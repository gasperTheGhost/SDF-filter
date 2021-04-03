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
    let refvalue: f64 = matches.value_of("value").unwrap().parse().expect("Value is not valid float!");
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
                let mut matching_records: Vec<String> = Vec::new();
                for block in sdf::prepare_file_for_SDF(&file) {
                    let mut record: SDFRecord = SDFRecord::new();
                    record.readRec(block);
                    let value = match record.getData(field).parse::<f64>() {
                        Ok(num) => Some(num),
                        Err(_) => None
                    };

                    // Get matching records
                    match evaluate(value, refvalue, operand) {
                        Some(result) => {
                            if result {
                                matching_records.push(record.lines.join("\n"));
                            }
                        },
                        None => ()
                    }
                }

                // Write vector of extracted data to stdout
                io::stdout().write_all((matching_records.join("\n$$$$\n")+"\n$$$$").trim().as_bytes()).expect("Error writing to stdout");
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
                let mut matching_records: Vec<String> = Vec::new();
                for block in sdf::prepare_file_for_SDF(&file) {
                    let mut record: SDFRecord = SDFRecord::new();
                    record.readRec(block);
                    let value = match record.getData(field).parse::<f64>() {
                        Ok(num) => Some(num),
                        Err(_) => None
                    };

                    // Get matching records
                    match evaluate(value, refvalue, operand) {
                        Some(result) => {
                            if result {
                                matching_records.push(record.lines.join("\n"));
                            }
                        },
                        None => ()
                    }
                }

                // Set output path
                let out_path: String = match file.trim() {
                    "-" => (output.to_owned() + "/stdin.txt"),
                    _ => (output.to_owned() + "/" + (&file.split("/").collect::<Vec<&str>>()).last().unwrap()),
                };
                // Write vector of extracted data to file
                sdf::write_to_file(&((matching_records.join("\n$$$$\n")+"\n$$$$").trim()), &out_path);
            }).collect();

            println!("Done in {}", HumanDuration(started.elapsed()));
        }

    } else {
        // Exit if input doesn't exist
        process::exit(0x0100);
    }
}

fn evaluate(value: Option<f64>, refvalue: f64, operand: &str) -> Option<bool> {
    match value {
        None => return None,
        Some(value) => {
            match operand {
                "lt" => {
                    if value < refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                "le" => {
                    if value <= refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                "eq" => {
                    if value == refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                "ne" => {
                    if value != refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                "ge" => {
                    if value >= refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                "gt" => {
                    if value > refvalue {
                        return Some(true);
                    } else {return Some(false)}
                },
                _ => {
                    eprintln!("Unsupported operand!");
                    return Some(false);
                }
            }
        }
    }
}