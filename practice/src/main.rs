use std::io;
use practice::{Tweet,NewsArticle,Summary,notify};

fn main(){
  
}

fn largest<T>(list:&Vec<T>)->T{
  let mut largest=list[0];

  for &item in list{
    if item>largest{
      largest=item;
    }
  }
  largest
}