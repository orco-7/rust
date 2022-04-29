use std::fs::File;
use std::io::{self, Read};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)

    //a more efficient way to implement the same functionality would be
    // fs::read_to_string("test.txt")  //no semicolon
}

pub fn last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}