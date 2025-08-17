fn main(){
    let mut s1: String = String::from("Everest");
   let s2=&mut s1;
   s2.push_str("Paudel");
    s1.push_str("Paudel");
    println!("s1 ={}",s1);
    // println!("s1 ={}",s2);
}
