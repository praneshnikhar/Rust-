struct Rect{
    width:u32 ,
    height:u32,
}

impl Debug for Rect{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"the reactangle prints like this{}", self.width * self.height)
    }
}

fn main(){
    let rect = Rect{
        width:30,
        height:50,        
    };
    print!("The area of the rectangle is {:?}", rect.area());// to debug u ned to add colon and question mark to debug
    print!("The area of the rectangle is{}", rect.perimeter());
}
