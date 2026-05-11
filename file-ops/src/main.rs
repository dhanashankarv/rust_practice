use std::io::{BufReader, BufRead, BufWriter, Write, Error, ErrorKind, SeekFrom, Seek};
use std::path::Path;
use std::env::args;
use std::process::exit;
use std::fs::{File, OpenOptions};
use rand::prelude::*;
use std::result::Result;
use std::str::FromStr;

fn gen_random(count: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut rvec: Vec<i32> = Vec::new();
    for _ in 0..count {
        rvec.push(rng.random::<i32>() % 1000);
    }
    return rvec
}

fn read_file(mut f: &File) -> Result<Vec<i32>, Error>{
    let reader :BufReader<&File>;
    let mut val :Vec<i32> = Vec::<i32>::new();
    f.seek(SeekFrom::Start(0))?; 
    reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(l) => {
                for s in l.split(",") {
                    if s.trim().is_empty() {
                        continue;
                    }
                    match i32::from_str(s) {
                        Ok(n) => val.push(n),
                        Err(e) => {
                            println!("Non numeric string {}: {}", s, e);
                            return Err(Error::from(ErrorKind::InvalidData));
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(val)
}

fn validate_filepath(fname: &String, read_only: bool) -> Result<File, std::io::Error> {
    let fpath = Path::new(fname);
    let res :Result<File, Error>;
    let err :Error;
    if read_only && !fpath.exists() {
        err = Error::from(ErrorKind::NotFound);
        return Err(err);
    }
    if !read_only {
        res = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(fname);
    } else {
        res = OpenOptions::new()
                .read(true)
                .open(fname);
    }
    return res;
}

fn write_file(f: &File, data: Vec<i32>) -> Result<(), std::io::Error>{
    let mut writer = BufWriter::new(f);
    for val in data.iter() {
        let _ = write!(writer, "{},", val);
    }
    writer.flush()?;
    println!("Wrote data into file");
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let tgt_file: File;
    let fname :&String;
    let rvec :Vec<i32>;
    let mut fvec :Option<Vec<i32>> = None;
    let cliargs: Vec<String> = args().collect();

    if cliargs.len() < 2 {
        eprintln!("Usage {} <filepath>", cliargs[0]);
        exit(-1);
    }

    fname = &cliargs[1];
    println!("Got file name {}", fname);

    match validate_filepath(fname, false) {
        Err(e) => {
            eprintln!("Invalid file path {}: {}", fname, e);
            exit(-2);
        },
        Ok(f) => tgt_file = f,
    }
    rvec = gen_random(10);
    println!("Got {} random numbers", rvec.len());
    for i in 0..rvec.len() {
        print!("{} ", rvec[i]);
    }
    println!("\n");

    write_file(&tgt_file, rvec)?;
    match read_file(&tgt_file) {
        Ok(val) => fvec = Some(val),
        Err(e) => eprintln!("File read failed {}", e),
    }

    println!("Read data from file: ");
    let val :Vec<i32> = fvec.expect("Read failed");
    for v in val {
        print!("{} ", v);
    }
    println!("");
    Ok(())
}
