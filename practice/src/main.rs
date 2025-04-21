use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number=rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}",secret_number);

    println!("Plewase input your guess.");

    let mut guess=String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}",guess);

    /*match guess.cmp(&secret_number){
        Ordering::Less=>println!("Too small!"),
        Ordering::Greater=>printnl!("Too big!"),
        Ordergin::Equal=>println!("You win!"),
    }*/
}
