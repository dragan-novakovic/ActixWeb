extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Gess the Number!!!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("Secret Number is :{}", secret_number);

    loop {
        println!("Input Your Number!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read Msg!!!!!");

        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Gessed: {}!", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
    }
    }

}
