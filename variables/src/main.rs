fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    another_function(x)
}

fn another_function(x: i32){
    println!("The value of x is: {}", x);
}
