// there can be many immutabel reference

fn main(){
    let s1 = String::from("hello");
    let s2= &s1;
    let s3= &s1;
    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);

}
