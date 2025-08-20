pub trait Summary {
    
    fn  summarize(&self) -> String;
}

struct  User{
    name:String,
    age:u32,
}

impl  Summary for User {
    fn  summarize(&self) -> String {
        return  format!("Name ={} age ={}",self.name ,self.age);
    }
    
}

fn main(){
let user = User{
    name:String::from("Everest Paudel"),
    age:32
};
print!("{}" ,user.summarize());
}
