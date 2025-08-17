
use std::collections::HashMap;


fn grouped_by_keypair(pairs:Vec<(String , i32)>)->HashMap<String ,Vec<i32>>{
   let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    // Insert a key with an empty Vec if it doesn't exist, then push value

    for (key ,value) in pairs {
    map.entry(key).or_insert(Vec::new()).push(value);
    
    }
    return map;
}




fn main(){
 let pairs = vec![
 (String::from("Everest "),22),
 (String::from("Everest "),21)
 ];
 
 let hm = grouped_by_keypair(pairs);

     println!("Heelo world this is from the rust {:?}",hm);
}
