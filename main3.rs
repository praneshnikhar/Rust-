fn main(){
    let greeting: String = String::from("hello world");
    println!("{}", greeting);
    let char1:Option<char> = greeting.chars().nth(0);
    print!("{}", char1);
}
