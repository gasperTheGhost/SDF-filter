#![allow(non_snake_case)]

use std::{
    io,
    fs::{self, metadata},
    process,
    time::Instant
};
extern crate clap;
use clap::{load_yaml, App};
extern crate walkdir;
use walkdir::WalkDir;
extern crate indicatif;
use indicatif::{HumanDuration, ParallelProgressIterator, ProgressBar, ProgressStyle};
extern crate rayon;
use rayon::prelude::*;

fn main() -> io::Result<()>{
    // Collect help information and arguments
    let yaml = load_yaml!("help/sddistill.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let filetypes: Vec<&str> = vec!["test"]; //matches.values_of("filetypes").unwrap().collect();

    // Iterate over input files 
    let files: Vec<String>;
    if let Some(path) = matches.value_of("input") {
        if path == "-"{ // If input is "-" then read from stdin
            files = vec![(&path).to_string()];
        } else if metadata(&path).unwrap().is_dir() { // Check if path points to dir
            files = getFiles(&path, filetypes.clone(), (&matches.is_present("recursive")).to_owned());
        } else { // Check if path points to a file
            files = vec![(&path).to_string()];
        }

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
            let contents = sdf::read_to_string(&file);

            // Split contents to vector of SDRecords (as strings)
            let mut contentVec: Vec<&str> = contents.split("$$$$").collect();
            contentVec.pop(); // Remove last (empty) item
            
            // Iterate over SDRecords
            let mut output: Vec<String> = Vec::new();
            for block in contentVec {
                // Get specified field from record and store value in vector
                output.push(extractData(block, matches.value_of("pattern").unwrap()).to_string());
            }
            // Set output path
            let out_path: String = match file.trim() {
                "-" => (matches.value_of("output").unwrap().to_owned() + "/stdin.txt"),
                _ => (matches.value_of("output").unwrap().to_owned() + "/" + (&file.split("/").collect::<Vec<&str>>()).last().unwrap()),
            };
            // Write vector of extracted data to file
            sdf::write_to_file(&(output.join("\n")), &out_path);
        }).collect();
        // pb.finish();

        // Print time elapsed
        println!("Done in {}", HumanDuration(started.elapsed()));
        Ok(())
    } else {
        // Exit if input doesn't exist
        process::exit(0x0100);
    }

}

fn getFiles(path: &str, _filetypes: Vec<&str>, recursive: bool) -> Vec<String> {
    println!("Making list of files in directory...");
    
    let mut output: Vec<String> = Vec::new();
    if recursive { // Use WalkDir for recursive indexing
        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() && !entry.path().to_str().unwrap().contains("/.")  {
                output.push(entry.path().to_str().unwrap().to_owned());
            }
        }
    } else { // Index files in direcory non-recursively
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() && !entry.path().to_str().unwrap().contains("/.") {
                output.push(entry.path().to_str().unwrap().to_owned());
            }
        }
    }
    return output;
}

fn extractData(sdfblock: &str, pattern: &str) -> String {
    let firstSplit: Vec<&str> = sdfblock.split(pattern).collect();
    if firstSplit.len() < 2 {
        println!("Data field not found in block\n{}\n", &sdfblock);
        return "N/A".to_owned();
    } else {
        return firstSplit[1].split(">").collect::<Vec<&str>>()[1].trim().to_string();
    }
}