fn main() {
    println!("Hello, world!");
    println!("The first word of the string {}",return_first_word(String::from("EVERES PAUDEL")))
}

fn return_first_word(s:String)->String{
    let mut ans = String::from("");
    for i in s.chars(){
        if i == ' '{
            break;
    }

    ans.push_str(&i.to_string());
}
return  ans;
}
   
