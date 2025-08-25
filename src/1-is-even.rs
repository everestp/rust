fn main() {
    let ans = is_even(1120);
    println!("{}",ans);
}

//function to check the number is even or odd
// i32 is signed which can give + -
fn is_even(num:u32)-> bool{
    if num %2 ==0 {
        return true;
    }
    else {
        return false;
    }

    
}

fn is_odd(num:u32)-> bool{
    if num %2 ==0 {
        return false;
    }
    else {
        return true;
    }
