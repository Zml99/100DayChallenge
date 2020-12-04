fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    let mut s = String::from("hello");

    let r1 = &mut s;
    //Can't have two reference mut variables
    //let r2 = &mut s;

    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}