fn main() {

    // Floating point numbers
    let x = 2.0;
    let y: f32 = 3.0;

    // Numerical Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false;

    // Chars
    let c = 'z';
    let z: char = 'ℤ';
    let cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6. , 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // Tuples: indexing
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    
}
