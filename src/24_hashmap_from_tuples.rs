use std::collections::HashMap;


fn group_values_by_keys(pairs:Vec<(String ,i32)>)-> HashMap<String,i32>{
 let mut hm = HashMap::new();
 
 for (key ,value) in pairs {
     hm.insert(key ,value);
 }
return hm
    
}

fn main(){
    let input_vec:Vec<(String,i32)> = vec![
    (String::from("Everest"),22),
    (String::from("Solana"),19)
    ];
    
    
    let hm =group_values_by_keys(input_vec);
   
        println!("{:?}",hm);
    
}
