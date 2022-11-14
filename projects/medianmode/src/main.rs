fn main() {
    let mut numbers = vec![2, 5, 4, 1, 5, 6, 9];

    get_median(&mut numbers);
    get_mode(&mut numbers);
}

fn get_median(numbers: &mut Vec<i16>) {
    
    numbers.sort();

    let middle_index = numbers.len() / 2;
    println!("Median: {}", numbers[middle_index]);
}

fn get_mode(numbers: &mut Vec<i16>) {

    use std::collections::HashMap;

    numbers.sort();

    let mut map: HashMap<i16, i16> = HashMap::new();

    for number in numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    } 

    println!("{:?}", map);
}
