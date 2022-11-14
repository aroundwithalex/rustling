fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}

fn find_until(v: &Vec<i32>, n: i32, til: usize) -> Option<usize> {
    for i in 0 .. til {
        if v[i] == n {
            return Some(i);
        }
    }

    return None;
}
