fn main() {
    println!("Hello, world!");
    let index = find_first_a(String::from("Everest"));
    if index == -1 {
        println!("a not found");
    }else {
        println!("postion at  as {}",index+1);
    }
 
}

fn find_first_a(s:String)-> i32 {

 for(index ,char) in s.chars().enumerate(){
     if char =='a' {
         return index as i32;
     }
 }
 return -1
 }
