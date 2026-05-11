use std::env;
use std::path;
use std::fs;

fn main() {
    let cliargs: Vec<String> = env::args().collect();
    if cliargs.len() < 3 {
        eprintln!("Usage: {} <pattern> <replacement> <files...> ", cliargs[0]);
        std::process::exit(-1);
    }

    let pattern: &String = &cliargs[1];
    let replacement: &String = &cliargs[2];
    let filenames = &cliargs[3..];

    println!("Rename filenames with pattern '{}' to '{}'", pattern, replacement);
    for fname in filenames {
        let fpath = path::Path::new(fname);
        if !fpath.exists() {
            eprintln!("File '{}' does not exist", fname);
            continue;
        }
        let newname = String::from(fname).replace(pattern, replacement);
        print!("Rename '{} -> {}' ", fname, newname);
        match fs::rename(fpath, path::Path::new(&newname)) {
            Ok(_) => println!(" successful"),
            Err(e) => println!("failed: {}", e),
        }
    }
    println!("\n");

}

