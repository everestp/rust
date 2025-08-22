


trait Summary {
  fn summarize(&self)->String;
}
struct NewArticle{
  headline:String,
  location:String,
  author:String,
  content:String
}

struct Tweet{
 username:String,
 content:String,
 reply:String,
 retweet:String
}
 impl Summary for Tweet {
     fn summarize(&self)->String{
 let content = format!("Tweet by {} :{}",self.username ,self.content);
 content
     }
 }

 impl Summary for NewArticle {
     
     fn summarize(&self)->String {
         let content = format!("New Artcile by :{} {}",self.author ,self.content);
         content
     }
 }



fn news_aggreator(source:impl  Summary){
  println!("There is a new news in the market");
  println!("{}",source.summarize());
}




fn main(){
let  tweet = Tweet{
  username: String::from("everst_paudel"),
  content:String::from("The exicited blockchain product is here a statup"),
  reply:String::from("All the best for the startup"),
  retweet:String::from("Oh really something new is coming"),

};
let  new_article: NewArticle = NewArticle{
  author: String::from("everst_paudel"),
  headline:String::from("The exicited blockchain product is here a statup"),
  location:String::from("All the best for the startup"),
  content:String::from("Oh really something new is coming"),

};
news_aggreator(tweet);
news_aggreator(new_article);
} 
