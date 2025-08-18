fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.into_iter(); // consumes v1 ❗
    
    for val in v1_iter {
        println!("{}", val);
    }
    println!("{:?}", v1); // ❌ ERROR: v1 was moved above
}
/*
into_iter() takes ownership of the vector.

That means after creating v1_iter, the original v1 is gone (moved).

So when you try to println!("{:?}", v1), Rust complains:

“borrow of moved value: v1”




⚡ Rust Rule of Thumb:

iter() → borrow, keeps vector alive.

iter_mut() → borrow mutably, allows modifying.

into_iter() → consumes vector, no going back.




*/
