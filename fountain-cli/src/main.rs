mod error;

use error::FountainError;
use fountain;
use nom::error::ErrorKind;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const ERR_UNPARSED: &str = "Parsing stopped before the document ended. Check the formatting of the following section. Unparsed text";

fn main() -> Result<(), FountainError> {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        println!("{}", fountain_to_html(path)?)
    } else {
        eprintln!("Missing FILEPATH arg");
        eprintln!("usage: $ fountain FILEPATH");
    }
    Ok(())
}

// Parse the .fountain file at the given filepath into HTML.
fn fountain_to_html(filepath: &str) -> Result<String, FountainError> {
    eprintln!("Reading {}", filepath);
    let text = read(filepath)?;
    match fountain::parse_document::<(&str, ErrorKind)>(&text) {
        Err(e) => Err(FountainError::ParseError(format!("{:?}", e))),
        Ok(("", parsed)) => Ok(parsed.as_html()),
        Ok((unparsed, parsed)) => {
            eprintln!("{}: {}", ERR_UNPARSED, unparsed);
            Ok(parsed.as_html())
        }
    }
}

// Read a file's contents into a string
fn read(filepath: &str) -> Result<String, io::Error> {
    let mut f = File::open(filepath)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
