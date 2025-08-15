struct Rect{
    length:i32,
    breadth:i32,
}

impl Rect{
 // &slef is like this keyword in javascript
 //if there is no &slef it is like static function and should diretly call by class not form object
    fn area(&self)->i32{
        self.length * self.breadth

    }
    fn debug()->i32{
        return 1
    }
    fn perimetre(&self)->i32{
        2 *(self.length + såelf.breadth)
    }
}

fn main() {
    let rect = Rect{
        length:12,
        breadth:20,
    };
    println!("hello, world");
    println!("The area of rectangle {}",rect.area());
    println!("The permimetre of the rectangle {}",rect.perimetre());
    println!("The debug function have no &self so it doesnot exist in the scope so called directly by class not object {}",Rect::debug());
    
    
    
    
    
    
    
}
