fn main() { //s is not declare over here
    //let s = "hello"; //s is valid from this point

    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);



    //If you want to deeply copy the heap data of a string
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    
} //The scope is over so s is no valid over here
