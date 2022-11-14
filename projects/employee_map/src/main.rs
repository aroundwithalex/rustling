use std::collections::HashMap;

fn main() {

    let mut map: HashMap<&str, &str> = HashMap::new();
    print_employee_records(&mut map, "Alice", "Engineering");

}

fn print_employee_records<'a>(map: &mut HashMap<&'a str, &'a str>, name: &'a str, dept: &'a str) {
    map.insert(name, dept);
    println!("{:#?}", map);
}
