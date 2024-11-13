struct User{
    name:String,
    age:String,
    active: boo,

}
fn main(){
    //tuple struct, unit struct
    let name= String::from("Pranesh ");
    let age = 30;
    let active = true;
    let user = User{
       
    };
    println!("{} is {} year old", user.name, user.age)
}
