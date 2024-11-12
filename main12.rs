fn main(){
    create_str();
    print!("{}",2);
}

fn create_str(){
    let s1 = String::from("Hi there");
    println!("{}", s1);
    let s2= s1;
    println!("{}",s2);

}
