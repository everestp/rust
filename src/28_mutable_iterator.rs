

fn main(){
  
   let mut v1  = vec![1,2,4,5];
   
   let v1_iters = v1.iter_mut();
   for val in v1_iters{
       *val = *val + 1;
        println!("{}",val);
   }
    println!("{:?}",v1);
}

/*

 Rust Borrowing Tip: When do you need *?
In Rust, references behave smartly:
✅ Reading (arithmetic, printing, comparing) → auto-deref works.
 ✍️ Writing/Mutating → you must explicitly use *.

*/
