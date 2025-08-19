
//Write a program to first filter all odd value then double each value and crate a new vector 


fn main() {
   let   v1 = vec![1,2,3,4,5];
   let  mut new_vec = Vec::new();
   for  i in v1.iter().filter(|x| *x %2 ==0 ).map(|x| x*2 ) {
   
     new_vec.push(i);
   }
   let   vec = vec![1,2,3,4,5,6];
   let ans = filter_and_map(vec.clone()); // use clone because we use it line 16  to again call the function while calling by actaul it transfer the ownership
   println!("This is the answer->{:?}",ans);
   println!(" This is the new_vec{:?}",ans);
   println!(" Can we  directly call and print let see{:?}",filter_and_map(vec));
     println!(" Yes we can {:?}",ans);
}



fn filter_and_map( vec:Vec<i32>)->Vec<i32>{
    let  new_iter = vec.iter().filter(|x| *x %2==0).map(|x| x*2);
    let new_vec :Vec<i32> = new_iter.collect();
    return  new_vec;
} 
