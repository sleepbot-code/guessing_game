use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn matching(guessed: u32, secret_number: u32){
    match guessed.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}


fn main() {
    loop{
        println!("Guess: ");
        let secret_number = rand::thread_rng().gen_range(1..=10);
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail");
        let guess: u32 = guess.trim().parse().expect("Enter number!");

        matching(guess, secret_number);
        if guess == secret_number{
            break;
        }
    }
}
