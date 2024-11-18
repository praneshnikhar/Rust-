use std::fs;

enum Result<A,B>{
    Ok(A), 
    Err(B),
 }
 
 fn main(){

     let res = fs::read_to_string("pranesh.txt".to_string());
 
     match res {
        Ok(content)=>{
            println!("File content: {}", content);
        },
        Err(err)=>{
            print!("Error: {}", err);
        }
    
    
     }
     print!("hi there")
 
 }

 fn read_from_file_unsafe(file_content:String)->RsultString{
    let res = fs::read_to_string("pranesh.txt");
    return res.unwrap();    
 }

