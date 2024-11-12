// note there cna only be one mutable reference at a time
// and also if u have a mutable reference u cannot have another immmutable reference

fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    let s3 = &s1;
    
    println!("{}", s1);
    println!("{}", s2);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
