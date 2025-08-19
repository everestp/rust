

use std::collections::HashMap;



fn main(){
    // Create a Hashmap and populate it  with some  key-valuer pairs

    let mut score: HashMap<&'static str, i32> = HashMap::new();
    score.insert("Alice", 20);
     score.insert("Alice", 20);
      score.insert("Everest", 20);
       score.insert("Alice", 20);

       //Example 1 :Iterating over reference to key-value pairs
       println!("Iterating Over key-value pairs:");
       for(key ,value) in score.iter(){
        println!("{}:{}",key,value);
       }

       //Example 2: Iterating over mutabloe reference to key-value pairs

    println!("Iterating over mutable key-value pairs");
    for(key ,value) in score.iter_mut(){
        *value +=10;
        println!("{}:{}",key,value);
    }
    println!("{:?}",score);
}
