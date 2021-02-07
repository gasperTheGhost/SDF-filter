extern crate clap;
use clap::{load_yaml, App};

fn main(){
    let yaml = load_yaml!("help/sdsplit.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let name = matches.value_of("name").unwrap();
    match matches.is_present("number_of_files") {
        true => split_package_num(&input, matches.value_of("number_of_files").unwrap().parse::<usize>().unwrap(), output, name),
        false => split_package_size(&input, matches.value_of("size_of_files").unwrap().parse::<usize>().unwrap(), output, name)
    }
}

fn split_package_num(package: &str, threads: usize, outputdir: &str, filename: &str) {
    let separator = "\n$$$$";
    let contents = sdf::read_to_string(package);
    let mut content_iterator: Vec<&str> = contents.split("\n$$$$").collect();
    content_iterator.pop();

    let div: usize = content_iterator.len() / threads;
    let rem: usize = content_iterator.len() % threads;
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

    let mut current = 0;
    for itm in &temp {
        let mut content: String = String::new();
        let mut n = 0;
        while &n < itm {
            content = content.to_owned() + &(content_iterator[current as usize].to_owned() + separator);
            n = n + 1;
        }
        sdf::write_to_file(content.trim(), &(outputdir.to_owned() + "/"+ &filename + &(current+1).to_string() + ".sdf"));
        current = current + 1;
    }
}

fn split_package_size(package: &str, size: usize, outputdir: &str, filename: &str) {
    let separator = "\n$$$$";
    let contents = sdf::read_to_string(package);
    let mut content_iterator: Vec<&str> = contents.split("\n$$$$").collect();
    content_iterator.pop();
    
    let mut files: Vec<String> = Vec::new();
    let mut n = 0;
    let mut content: String = String::new();
    for block in content_iterator {
        if n < size {
            content = content.to_owned() + &(block.to_owned() + separator);
            n = n + 1;
        } else {
            files.push(content.clone());
            content = block.to_owned() + separator;
            n = 1;
        }
    }
    files.push(content.clone());

    let mut current = 0;
    for file in files {
        sdf::write_to_file(file.trim(), &(outputdir.to_owned() + "/"+ &filename + &(current+1).to_string() + ".sdf"));
        current = current + 1;
    }
}