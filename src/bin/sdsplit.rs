use std::{
    env,
    fs,
    process
};

// Usage: sdsplit path/to/package.sdf intOfPackages path/to/output/dir

fn main(){
    let args: Vec<String> = env::args().collect();

    let outputdir: &str;
    if args.len() == 4 {
        outputdir = &args[3];
    } else if args.len() == 3 {
        outputdir = ".";
    } else {
        println!("Invalid arguments! Cannot split SDF package!");
        process::exit(0x0100);
    }
    split_package(&args[1], args[2].parse::<usize>().unwrap(), outputdir);
    
}

fn split_package(package: &str, threads: usize, outputdir: &str) {
    let separator = "\n$$$$\n";
    let contents = fs::read_to_string(package)
        .expect("Error reading input SDF file!");
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
        sdf::write_to_file(&("\n".to_owned() + &content.trim()), &(outputdir.to_owned() + "/temp" + &(current+1).to_string() + ".sdf"));
        current = current + 1;
    }
}