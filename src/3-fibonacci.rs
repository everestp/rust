fn main(){
println!("{}",fibo_index(5))
}

fn fibo_index(num:i32)->i32{
    let mut first_num=01;
    let mut second_num=1;
  if num==0{
    return first_num;
  }
    if num==1{
        return 1;
    } 

for i in 1..num-2 {
    let temp = second_num;
    second_num =second_num +first_num;
    first_num= temp
}
 
return second_num;
}