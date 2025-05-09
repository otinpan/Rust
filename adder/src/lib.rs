pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works()->Result<(),String>{
        if 2+1==4{
            Ok(())
        }else{
            Err(String::from("err"))
        }
    }
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width&&self.height>other.height
    }
}
