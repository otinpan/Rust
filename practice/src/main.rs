use std::io;
fn main(){
    let mut input=String::new();
    let mut num:u32;

    //数値の入力
    loop{
        println!("please input number");

        io::stdin()
        .read_line(&mut input)
        .unwrap();
    
        num=input.trim().parse().expect("can not convert");

        if num<=0{
            continue;
        }else{
            break;
        }
    }

    if num==1||num==2{
        println!("1");
        return;
    }
    let mut pre=1;
    let mut ppre=1;
    let mut now=2;
    for i in 2..num{
        let keep=now;
        now=pre+ppre;
        ppre=pre;
        pre=now;
    }

    println!("{}",now);
}