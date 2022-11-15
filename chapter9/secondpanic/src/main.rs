use std::fs::File;
use std::error::Error;

fn main() {
    println!("{}", read_username_from_file().unwrap());
}

fn read_username_from_file() -> Option<String> {
    let mut username_file = File::open("hello.txt");
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Some(username)
}
