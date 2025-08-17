

fn main(){
  
   let v1  = vec![1,2,4,5];
   
   let  mut v1_iters = v1.iter();
  println!("{:?}",v1_iters);
  while let Some(val) = v1_iters.next(){
      println!("{}",val);
  }
    
}
