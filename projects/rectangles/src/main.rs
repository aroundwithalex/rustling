#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Point{
    x: i32, 
    y: i32} 

impl Point {
    fn get_x(&mut self) -> &mut i32 {
        &mut self.x
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.width());

    let sq = Rectangle::square(3);

    let mut p = Point{x: 1, y: 2};
    let x = p.get_x();
    *x += 1;
    println!("{} {}", *x, p.y);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
