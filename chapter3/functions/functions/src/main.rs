fn main() {
    print_labeled_measurement(5, 'h');
    
    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(1);
    println!("The value of x is: {y}");

    let z = {
        let x = 3;
        x + 1
    };

    println!("The value of z is: {z}");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
