
struct User{
    is_active:bool,
    name:String,
    email:String,
    sign_in_count:usize
}
fn main() {
    println!("Hello, world!");
    let user1 =User{
        is_active:true,
        name:String::from("Everest paudel"),
        email:String::from("everest@gmail.com"),
        sign_in_count :23,
    };
    println!("{}",user1.is_active);
    
}



