struct Point <T ,U,V>{
x:T,
y:U,
z:V


}


fn main() {
    let point= Point{
       x:2,
       y:2.1000000,
       z:String::from("Everest")
    
    };
    println!("Hello, world!");
    print!("Hello this is the generic :{}",point.z);
}
