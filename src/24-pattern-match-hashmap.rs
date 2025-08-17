use std::collections::HashMap;

fn main(){
    let mut users = HashMap::new();
    users.insert(String::from("Everest"),22);
    users.insert(String::from("raman"),32);
    /*
    what this hash map look like
       Everest :22,
       raman :32,
       
    
    */
    let first_user_age  = users.get("Everestg");  //option <22>
    
    match first_user_age{
        Some(age) => println!("age is {}",age),
        None => println!("User not found in the datrabases"),
    }
    
    println!("user age ={:?}",first_user_age);
}
