extern crate clap;
use clap::{load_yaml, App};

fn main(){
    // Collect help information and arguments
    let yaml = load_yaml!("help/sdsplit.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let name = matches.value_of("name").unwrap();

    // Check if package should be split by number of files or number of records
    match matches.is_present("number_of_files") {
        true => split_package_num(&input, matches.value_of("number_of_files").unwrap().parse::<usize>().unwrap(), output, name),
        false => split_package_size(&input, matches.value_of("size_of_files").unwrap().parse::<usize>().unwrap(), output, name)
    }
}

fn split_package_num(package: &str, threads: usize, outputdir: &str, filename: &str) {
    let separator = "\n$$$$";

    // Save file contents to string
    let contents = sdf::read_to_string(package);
    // Split file into vector of SDRecords (as strings)
    let mut content_iterator: Vec<&str> = contents.split("\n$$$$").collect();
    content_iterator.pop(); // Remove last (empty) vector item

    // Calculate number of SDRecords per file
    let div: usize = content_iterator.len() / threads;
    let rem: usize = content_iterator.len() % threads;

    // Store number of SDRecords in vector of ints
    let mut temp: Vec<usize> = Vec::new();
    let mut i = 1;
    while i <= threads {
        if i <= rem {
            temp.push(div + 1);
        } else {
            temp.push(div);
        }
        i = i + 1;
    }

    // Iterate over vector of SDRecords per file
    let mut current = 0;
    for itm in &temp {
        let mut content: String = String::new();
        let mut n = 0;
        // Store SDRecords in vectors of size itm
        while &n < itm {
            content = content.to_owned() + &(content_iterator[current as usize].to_owned() + separator);
            n = n + 1;
        }
        // When vector size is reached, write it to a new file
        sdf::write_to_file(content.trim(), &(outputdir.to_owned() + "/"+ &filename + &(current+1).to_string() + ".sdf"));
        current = current + 1;
    }
}

fn split_package_size(package: &str, size: usize, outputdir: &str, filename: &str) {
    let separator = "\n$$$$";

    // Save file contents to string
    let contents = sdf::read_to_string(package);
    // Split file into vector of SDRecords (as strings)
    let mut content_iterator: Vec<&str> = contents.split("\n$$$$").collect();
    content_iterator.pop(); // Remove last (empty) vector item
    
    // Iterate over vector of SDRecords
    let mut files: Vec<String> = Vec::new();
    let mut n = 0;
    let mut content: String = String::new();
    for block in content_iterator {
        if n < size {
            // Append SDRecord (as string) to string of SDRecords if number of records is <= specified size
            content = content.to_owned() + &(block.to_owned() + separator);
            n = n + 1;
        } else {
            // Push string to vector of contents when size exceeds specified size
            files.push(content.clone());
            content = block.to_owned() + separator;
            n = 1;
        }
    }
    // Append overflow SDRecords to vector of contents
    files.push(content.clone());

    // Write each element in vector of contents to new file
    let mut current = 0;
    for file in files {
        sdf::write_to_file(file.trim(), &(outputdir.to_owned() + "/"+ &filename + &(current+1).to_string() + ".sdf"));
        current = current + 1;
    }
}