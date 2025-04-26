use std::io;
fn main(){
  let s=String::from("hello world");

  let k=first_word1(&s);
 
  println!("{}",k);
}

fn first_word1(s: &str)->&str{
  let bytes=s.as_bytes();
  for (i,&item) in bytes.iter().enumerate(){
    if item==b' '{
      return &s[0..i]
    }
  }
  &s[..]
}
