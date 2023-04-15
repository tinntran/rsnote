use std::fs::{self, File};
use std::io::{self, ErrorKind, prelude::*};
use crate::DATAPATH;

pub fn read_file() -> Result<String, io::Error> {
    fs::read_to_string(DATAPATH)
}

pub fn write_file(content: &str) -> Result<(), io::Error> {
    let mut file = File::create(DATAPATH)?;
    file.write_all(content.as_bytes())?;
    file.sync_data()?;

    Ok(())
}


pub fn read_or_create_file() -> Vec<String> {
    let mut vec = Vec::new();

    let binding = read_file().unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => {
            println!("File not found. Creating file...");
            const SAMPLE: &str = "0|Create a todo|X";
            
            match write_file(SAMPLE) {
                Ok(()) => SAMPLE.to_string(),
                Err(e) => panic!("{}", e),
            }
        },
        other => panic!("Reading file error: {}", other),
    });

    let raw = binding.trim();

    let slice_vec = raw.split('\n').collect::<Vec<&str>>();

    for s in slice_vec {
        vec.push(s.to_string());
    }
    
    vec
}

