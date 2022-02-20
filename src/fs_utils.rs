use std::{fs::File, io::{Error, Read}};

pub fn read_file(file: &mut File) -> Result<String, Error> {
    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e)
    }
}