use std::io;
use std::thread;
use std::fmt::Display;

fn longest_with_ans_announcement<'a,T>(
  x:&'a str,
  y:&'a str,
 // ann: T,
)->&'a str
where T:Display
{
  //println!("{}",ann);
  if x.len()>y.len(){
    x
  }else{
    y
  }
}
fn main() {
  let s1=String::from("yes");
  let s2=String::from("no");
  let num:i64=89;

  let s1=longest_with_ans_announcement(&s1,&s2,num);

  println!("{}",s1)
}