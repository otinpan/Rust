use std::io;
use practice::{Tweet,NewsArticle,Summary};

fn main(){
  let tweet=Tweet{
    username:String::from("horse_ebooks"),
    content:String::from(
      "Hi"
    ),
    reply:false,
    retweet:false,
  };

  let news=NewsArticle{
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
  };
  println!("{}",news.summarize());
  println!("{}",tweet.summarize());
}

