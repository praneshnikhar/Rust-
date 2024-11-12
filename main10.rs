fn main(){
    let mut x: String = String::from("Hi there");//immutable =>can not be changed
    x.push_str("asd"); 
    //error beacuse x is immutable 
    //mutability leads to lot of memory issues.
    println!("{}", x);
}
