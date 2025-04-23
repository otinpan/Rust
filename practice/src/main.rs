use std::io;
fn main(){
    let condition=true;
    let number={
        if condition{
            println!("Yes");
            -5
        }else{
            println!("No");
            6
        }
    };
    println!("{}",number);
}