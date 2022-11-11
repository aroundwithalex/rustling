fn main() {
    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    let _s3 = takes_and_gives_back(s2);

    let s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of {} is {}", s4, len);

    let mut s5 = String::from("hello");
    print(&s5);
    s5.push_str(" world");
    print(&s5);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    let mut n = 1;
    incr(&mut n);
    println!("{n}");

    let reference_to_nothing = dangle();

    let s = String::from("hello world");
    let first_word = first_word(&s);
    println!("{}", first_word)
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn print(s: &String) {
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn incr(n: &mut i32) {
    *n += 1;
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
