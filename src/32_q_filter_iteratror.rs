//Write a program to first filter all odd value then double each value and crate a new vector 


fn main() {
   let mut  v1 = vec![1,2,3,4,5];
   let mut new_vec = Vec::new();
   for  i in v1.iter().filter(|x| *x %2 ==0 ) {
   
     new_vec.push(*i *2);
   }
   println!("{:?}",new_vec);
}
