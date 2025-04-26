use std::io;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
  let mut s=String::from("hello");
  let slice=&s;
  
  s.clear();
  println!("{}",slice);
}
