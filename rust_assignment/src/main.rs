// use std::fs;
// use std::fs::File;
// use std::io::Write;

// use std::io::{Read, BufReader, BufRead};
// use std::fs::OpenOptions;

use std::io::{self, Read, Write};

struct Person {
    name: String,
    sid: u32,
}

fn main() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    print!("{}", buffer);
    buffer.clear();

    print!("What's your student ID #? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let sid = buffer.trim().parse().unwrap();
    let person = Person { name, sid };
    buffer.clear();
    
    println!("Hi {}: your ID is {}!", person.name, person.sid);
}


