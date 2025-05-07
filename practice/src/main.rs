use std::io;
use practice::{Tweet,NewsArticle,Summary,notify};
use std::fmt::Display;

fn main(){
  let p=Pair::new(5,3);
  p.cmp_display();
}

struct Pair<T>{
  x:T,
  y:T,
}

impl<T> Pair<T>{
  fn new(x:T,y:T)->Self{
    Self{x,y}
  }
}

impl<T:Display+PartialOrd> Pair<T>{
  fn cmp_display(&self){
    if self.x>=self.y{
      println!("{}",self.x);
    }else{
      println!("{}",self.y);
    }
  }
}