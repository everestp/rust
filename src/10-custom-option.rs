enum CustomOption {
    Some(i32),
    None
}

fn main() {
   let index = find_first_a(String::from("pregdaet"));
    match index {
        CustomOption::Some(value)=>println!("index at {}",value+1),
        CustomOption::None => println!(" Letter is not found in the word")
    }
   
}


 fn find_first_a(s: String) -> CustomOption {
     for (index ,char) in s.chars().enumerate(){
        if  char =='a'{
            return CustomOption::Some(index as i32);
        }
        
         }
        return CustomOption::None;
     }
     
 
