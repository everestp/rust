fn main() {
    let mut s1: String = String::from("Everest");

    // First mutable reference to s1
    let s2 = &mut s1;
    s2.push_str(" Paudel");
    println!("s2 = {}", s2); // After this line, s2's borrow ends

    // Second mutable reference to s1 is allowed now
    let s3 = &mut s1;
    println!("s3 = {}", s3); // After this line, s3's borrow ends

    // Third mutable reference to s1 is allowed now
    let s4 = &mut s1;
    s4.push_str(" Rocks!");
    println!("s1 = {}", s4); // Use s4 to access and modify s1
}
