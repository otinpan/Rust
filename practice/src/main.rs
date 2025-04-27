use std::io;
#[derive(Debug)]
struct Rectangle{
  width:u32,
  height:u32,
}

impl  Rectangle{
  fn can_hold(&self,other:&Rectangle)->bool{
    self.width>other.width&&self.height>other.height
  }

  fn compare_area(&self,other:&Square)->bool{
    self.width*self.height>other.length*other.length
  }
}

#[derive(Debug)]
struct Square{
  length:u32,
}

impl Rectangle{
  fn square(size:u32)->Rectangle{
    Rectangle{width:size,height:size}
  }
}
fn main(){
  let sq=Rectangle::square(3);
}

