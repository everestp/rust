fn main(){
    let mut vec = vec![1,2,3,4,5,6]; //This is th ector macro in intilaize the vector 2035 aug 17 
    println!("{:?}",vec);
 let ans =  even_filter(&mut vec); // here i am passing the reference so  second function point to the reference can mutate the  vector
     println!("Upafda the same vector{:?}",vec);
     println!("Upafda the same vector{:?}",ans);
    
    
}

fn even_filter(vec: &mut Vec<i32>){
  let mut i =0;
     while  i < vec.len(){
         if vec[i] % 2 != 0 {
             vec.remove(i);
         }
         else{
             i +=1;
         }
     }
    
}
