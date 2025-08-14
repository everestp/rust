fn main() {
   
    
    println!(" is-odd {}",is_odd(1));

}

//function to check the number is even or odd
// i32 is signed which can give + -
 fn is_odd(num:i32)->bool {
    if num % 2 ==0 {
        return false;
    }else {
        return true;
    }
 }