fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let evens_refs: Vec<&i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .collect();

    // Show type information
    for r in &evens_refs {
        println!("Type: {}, Value: {}", 
            std::any::type_name::<&i32>(), r);
    }

    println!("Evens as refs: {:?}", evens_refs);
}
