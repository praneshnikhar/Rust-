fn main() {
    let greeting: String = String::from("hello world");
    println!("{}", greeting);
    let char1: Option<char> = greeting.chars().nth(1);
    print!("char1: {}", char1.unwrap());
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000"),
    }
}

