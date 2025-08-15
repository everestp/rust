enum Shape{
Rectangle(f64,f64), //length and breadth
Circle(f64), //Radius  of circle
Square(f64), //length
    
}



fn calculate_area(shape:Shape)->f64{
match shape {
    Shape::Rectangle(a,b)=>a*b,
    Shape::Circle(a)=> 3.14 * a *a,
    Shape::Square(a)=> a*a,
    
}

    
}
fn main(){
    let rectangle = Shape::Rectangle(5.0 ,3.0);
    println!("The area of rectangle={}",calculate_area(rectangle));
    let  circle = Shape::Circle(5.0);
    println!("The area of circle is {}",calculate_area(circle));
    let square = Shape:å:Square(7.0);
    println!("The area of square is {}",calculate_area(square));
}
    
