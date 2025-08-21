

struct Point<T ,U>{
    x:T,
    y:U
}


impl <T,U> Point<T ,U>{
    fn new1(x:T ,y:U)->Self{
Self { x, y}
    }
    
    fn mixup<X ,Y>(self ,point:Point<X,Y>)->Point<T ,Y>{
  Point { x: self.x, y: point.y }
    }
   
    
}


fn main(){

      let a: i32 = 10;  // i32 is Copy
    let b = a;         // Rust automatically copies the value
    println!("a = {}, b = {}", a, b); // ✅ a is still valid
    let point_a = Point {
        x:2,
        y:4.5
    };
      let point_b = Point::new1(3,1);
       

 let point_c=point_a.mixup(point_b);


}
