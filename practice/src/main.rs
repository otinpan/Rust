use std::io;
fn main(){
    for number in 1..4{
        println!("{}",number);
    }

    //reverseする
    for number in (1..4).rev(){
        println!("{}",number);
    }

    //1個とばし
    for number in (1..10).step_by(2){
        println!("{}",number);
    }
}