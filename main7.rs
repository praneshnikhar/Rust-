fn main() {
    // Define a sentence
    let sentence: String = String::from("My name is Pranesh");
    
    // Get the first word
    let first_word = get_first_word(&sentence);

    let n = 1000; 
    for i in 0..n { 
        println!("hello world! {}", i);
    }
    
    println!("\nFirst word is: {}", first_word);
}


fn get_first_word(sentence: &String) -> String {
    
    let mut ans = String::new();
    
    
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }
    
    ans
}

