use std::fs::File;
use std::io::{self,Read};

fn main() {
    let s=read_user_name_from_file();

}

fn read_user_name_from_file()->Result<String,io::Error>{
  let mut f=File::open("hello.txt")?;
  let mut s=String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}