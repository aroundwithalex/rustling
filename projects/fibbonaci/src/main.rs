use std::io;

fn main() {

    println!("Enter a number: ");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    
    let number: u32 = buffer
        .trim()
        .parse()
        .expect("Wanted a number");

    fibbonaci(number);
}

fn fibbonaci(n: u32) {
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    println!("{}", first);
    println!("{}", second);

    while second < n {
        let sum = first + second;
        first = second;
        second = sum;
        println!("{}", sum);
    }
}
