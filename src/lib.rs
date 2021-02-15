#![allow(non_snake_case)]

use std::{
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufReader},
    path::Path,
};
use walkdir::WalkDir;

pub fn getFiles(path: &str, _filetypes: Vec<&str>, recursive: bool) -> Vec<String> {
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

pub fn append_line_to_file(filename: &str, line: &str) {
    // Create path if inexistent
    let path = Path::new(filename);
    if filename.contains("/") {
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
    }

    // Open file, append line, close file
    let mut file = OpenOptions::new()
        .append(true)
        .open(&filename)
        .unwrap();
    if let Err(e) = writeln!(file, "{}", &line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn lines_from_file(filename: &str) -> Vec<String> {
    let mut reader: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(fs::File::open(Path::new(filename)).expect("No such file")))
    };
    let mut buf: Vec<u8> = Vec::new();
    let mut output: Vec<String> = Vec::new();
    while let Ok(_) = reader.read_until(b'\n', &mut buf) {
        if buf.is_empty() {
            break;
        }
        &buf.pop();
        if buf.last().unwrap().to_owned() == b'\r' {
            &buf.pop();
        }
        let line = String::from_utf8_lossy(&buf);
        output.push(line.into_owned());
        buf.clear();
    }
    return output;
}

pub fn read_to_string(filename: &str) -> String {
    let mut reader: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(fs::File::open(Path::new(filename)).expect("No such file")))
    };
    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf).expect("Cannot read file");
    let buf = String::from_utf8_lossy(&buf);
    return buf.into_owned();
}

pub fn write_to_file(content: &str, filename: &str) {
    let path = Path::new(filename);
    if filename.contains("/") {
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
    }
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}

pub fn write_bytes_to_file(content: Vec<u8>, filename: &str) {
    let path = Path::new(filename);
    if filename.contains("/") {
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
    }
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&content) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
}

pub fn prepare_file_for_SDF(file: &str) -> Vec<Vec<String>>{
    // Read file contents to string
    let contents = read_to_string(file);
    let mut contents_vec: Vec<&str> = contents.split("\n$$$$").collect();
    contents_vec.pop();
    
    // Iterate over SD Records (as strings)
    let mut output: Vec<Vec<String>> = Vec::new();
    let mut i = 0;
    for block in contents_vec {
        // Turn strings into a vector of lines
        let mut lines: Vec<String> = (block.to_string() + "\n$$$$").split('\n').map(|a|a.replace("\r","").to_string()).collect();
        if i == 0 {
            i = i + 1;
        } else {
            lines.remove(0);
        }
        output.push(lines);
    }
    return output;
}

pub mod sdfrecord {

    use std::io::{self, Write};
    use std::collections::BTreeMap;

/* 
 Methods:  new          - Constructor
           readRec      - Convert Vec of lines into SDF Record
           writeRec     - Write SDF Record to stdout
           writeData    - Output data key, val pairs to stdout
           copy         - Make a deep copy of SDF Record
           addData      - Add key, value pairs to SDF Record

 Data:     LINES    - Vec of all lines in the record
           DATA     - Map of record data. Everything except the mol is
                      stored here, the values are in Vecs to allow multiple lines
           DATAREF  - Map used as table of contents. Key stores the data title,
                      while value stores the line number, where it's located
*/

    pub struct SDFRecord {
        pub lines: Vec<String>,
        pub data: BTreeMap<String, Vec<String>>, // Should maybe be replaced by HashMap
        pub dataref: BTreeMap<String, usize>, // Should maye be replaced by HashMap
    }

    impl SDFRecord {
        /*
         Constructor
        */
        pub fn new() -> Self {
            return SDFRecord {
                lines: Vec::new(),
                data: BTreeMap::new(),
                dataref: BTreeMap::new()
            }
        }
        
        /*
         readRec() - read next SD record from vector of 
         Input params:
            file: Vec containing lines from SDF file
         Return:
            Vec<String> containing all lines after record separator
        */
        pub fn readRec(&mut self, file: Vec<String>) -> Vec<String> {
            // Clear old values
            self.lines.clear();
            self.data.clear();
            self.dataref.clear();

            let mut vector = file.to_owned();

            for line in &file {
                vector.remove(0);
                if line == "$$$$" {
                    break;
                } else {
                    self.lines.push(line.to_owned());
                }
            }

            if self.lines.len() > 0 {
                let mut fieldName: String = "".to_owned();
                let mut lineNum: usize = 0;
                let mut dataLines: Vec<String> = Vec::new();
                for line in self.lines.iter() {
                    lineNum += 1;
                    if lineNum <= 3 { // First three lines are reserved for TITLE
                        self.data.insert("_TITLE".to_string()+&(lineNum.to_string()),vec!(line.to_owned()));
                        if lineNum == 2 { // Include dimensionality as pseudo data field
                            let ndim: String = line.chars().skip(20).take(1).collect();
                            self.data.insert("_NDIM".to_string(),vec!(ndim));
                        }
                    } else if lineNum == 4 { // Include number of atoms as pseudo data field
                        let temp: Vec<&str> = line.split(" ").collect();
                        if temp.len() > 0 {
                            self.data.insert("_NATOMS".to_string(), vec!(temp[1].to_owned()));
                        }
                    }

                    // Found a data field
                    if line.find('>') == Some(0) {
                        let fieldNameVec: Vec<&str> = line.split(['<','>'].as_ref()).collect();
                        fieldName = fieldNameVec[2].to_string(); // Store field name
                        self.dataref.insert((&fieldName).to_string(), lineNum);
                    } else if &fieldName != "" { // If field name defined, store
                        if line != "" {
                            dataLines.push(line.to_string());
                        } else {
                            self.data.insert((&fieldName).to_string(), dataLines.clone());
                            dataLines.clear();
                            fieldName = "".to_owned(); // Clear field name
                        }
                    }
                }
            }

            /*
            println!("\nLine Vector");
            for line in &self.lines {
                println!("{}", line);
            }
            println!("\nData HashMap");
            for (key, val) in &self.data {
                println!("({},{:?})",key,val);
            }
            println!("\nDataref HashMap");
            for (key,val) in &self.dataref {
                println!("({},{})",key,val);
            }
            */

            return vector;
        }

        /*
         writeRec() - write current record to STDOUT (mol + data)
        */
        pub fn writeRec(&self) {
            if self.lines.len() > 0 {
                for line in &self.lines {
                    io::stdout().write_all(format!("{}\n",line).as_bytes()).expect("Error writing to stdout");
                }
                io::stdout().write_all(b"$$$$\n").expect("Error writing to stdout");
            }
        }

        /*
         writeMol() - write current mol record to STDOUT
        */
        pub fn writeMol(&self) {
            for line in &self.lines {
                io::stdout().write_all((line.to_string()+"\n").as_bytes()).expect("Error writing to stdout");
                if line == "M  END" {
                    break;
                }
            }
        }

        /*
         writeData() - list data field/values to STDOUT
        */
        pub fn writeData(&self) {
            for (key, val) in &self.data {
                // Output: $key eq "val1; val2"\n
                let output = String::from("$")+key+" eq "+"\""+&(val.join("; "))+"\"\n";
                io::stdout().write_all(output.as_bytes()).expect("Error writing to stdout");
            }
        }

        /*
         copy() - create deep copy of SDRecord
        */
        pub fn copy(&self) -> SDFRecord {
            let mut clone = SDFRecord::new();
            clone.lines = self.lines.clone();
            clone.data = self.data.clone();
            clone.dataref = self.dataref.clone();
            return clone;
        }
        
        /*
         addData() - adds data to data hash array,
         and adds corresponding lines also so that record may be
         rewritten with the new fields
         Input params:
            key: Title of data field
            value: Vec<String> of lines to be written in data field
        */
        pub fn addData(&mut self, key: String, value: Vec<String>) {
            if self.data.contains_key(&key) {
                let nrLines: usize = self.data[&key].len();
                let keyRef: usize = self.dataref[&key];
                self.data.insert(key.clone(), value.clone());
                for i in keyRef..(keyRef+&nrLines) {
                    self.lines.remove(i);
                }
                for val in value.iter().rev() {
                    self.lines.insert(keyRef, val.to_owned());
                }
            } else {
                self.dataref.insert(key.clone(), (&self.lines.len()).to_owned());
                self.data.insert(key.clone(), value.clone());
                self.lines.push(">  <".to_string()+&key+">");
                for line in value {
                    self.lines.push(line);
                }
                self.lines.push("".to_owned());
            }
        }

        pub fn getData(&self, key: &str) -> String {
            return match self.data.contains_key(key) {
                true => self.data[key].join(";"),
                false => "".to_owned()
            }
        }
    }

}