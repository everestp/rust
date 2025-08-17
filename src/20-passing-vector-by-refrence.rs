fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
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


 /*
    One of the simplest but most powerful lessons I’ve picked up in Rust 🦀
👉 Passing by mutable reference = you change the same data, nothing is returned.
 👉 Returning a value = you get a brand new copy to work with.
It’s such a small detail, but it forces you to be crystal clear:
 Am I mutating, or am I creating something new?
This mindset actually applies to life too — are you trying to change what exists, or create something new? 💡
    */
