use std::io;
use std::collections::HashMap;
fn main(){
  let text="hello world wonderful world";
  let mut map=HashMap::new();

  for world in text.split_whitespace(){
    let count=map.entry(world).or_insert(0);
    *count+=1;
  }

  println!("{:?}",map);
}
