fn main() {
    println!("Hello, world!");
   
    println!("the length of the string is the {}",get_string_length("Dfdfdf"))
}



//may be I am wrong 
// here the transer  of the owner of the string is main so passing the reference it mean  varaible is point to main heap of belong to main
fn get_string_length(str:&str)->usize{
str.chars().count()
}
