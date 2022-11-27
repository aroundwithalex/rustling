use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DICTIONARY: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Hello", "World");
        m.insert("Foo", "Bar");
        m.insert("Bar", "Baz");
        println!("Initialized");
        m
    };
}

fn main() {
    println!("Started....");
    println!("Dictionary contains {:?}", *DICTIONARY);
    println!("Dictionary contains {:?}", *DICTIONARY);
}
