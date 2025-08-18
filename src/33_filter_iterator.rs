fn main() {
    let v1  = vec![1,2,3,4];
    let v1_iter = v1.iter();
    let  v1_iter2 = v1_iter.filter(|x| *x % 2 ==0);
    for i in v1_iter2 {
        println!("{}",i);
    }
    println!("Hello, world!");
        println!("{:?}",v1);
}

/*
filter only needs to check a condition, it doesn’t need to change the item.

To avoid moving or copying unnecessarily, Rust passes a reference to the item (&Self::Item) to the closure.

That way, the original item stays in place, and filter just decides whether to keep it

*/
