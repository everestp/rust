fn main() {
    // Case 1: String literal → &'static str
    let x = "Everest";         // lives forever in program binary
    let s1 = x;                // just another reference (no copy)
    println!("x = {}, s1 = {}", x, s1);

    // Case 2: Owned String → heap allocated
    let y = String::from("Everest");  // owns its memory on heap
    let s2 = &y;                      // borrow reference
    println!("y = {}, s2 = {}", y, s2);

    // y is dropped here → heap memory freed automatically
}
