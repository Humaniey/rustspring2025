use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2ver() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {

    let res = read_username_from_file();
    println!("{:?}", res);

    let res = read_username_from_file_2ver();
    println!("{:?}", res);
    // let f = File::create("hello.txt");
    // let f = File::open("hello.txt");
    // println!("{:?}", f);

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    // println!("{:?}", f);
}
