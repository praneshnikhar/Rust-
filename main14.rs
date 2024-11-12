fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);  // Mutable borrow to `update_word`
    println!("{}", s1);     // Use `s1` directly here
}

fn update_word(word: &mut String) {
    word.push_str(" world");
}
