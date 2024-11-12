fn main(){
    let my_string = String::from("hello");
    let my_string = takes_ownership(my_string);
    println!("{}", my_string); //this line will cause a compile error beacuse the ownership as been moved
    println!("{}",my_string);
}   
   
    fn takes_ownership(some_string:String)-> String{
        println!("{}", some_string);//some_string now owns the data
        return some_string;
    }
