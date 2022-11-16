fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self. other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

enum Option<T, E> {
    Some(T),
    Err(E),
}

fn print_slice<T>(v: &[T]) {
    for x in v {
        println!("{x}");
    }
}

fn main() {
    print_slice(&[1, 2, 3]);
}
