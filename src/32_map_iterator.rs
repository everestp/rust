fn main() {
    let v1 = vec![1, 2, 3];
    let iter1 = v1.iter();
    let iter2 = iter1.map(|x| x+1);
    
    for x in iter2 {
        println!("{}" ,x);
    }
     println!("{:?}" ,v1);
    
   
}


/*
v1.iter() gives you an iterator over &i32.

map(|x| x+1) wraps that iterator in a Map struct, which is also an iterator.

Nothing runs yet! It’s lazy — the transformation only happens when you actually consume it (e.g., with for, .collect(), .next(), etc.).



*/
