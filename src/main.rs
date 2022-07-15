use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn matching(guessed: u8, secret_number: u8){
    match guessed.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}


fn main() {
    let mut guesses: Vec<u8> = Vec::new();
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail");
        let guess: u8 = guess.trim().parse().expect("Enter number!");
        guesses.push(guess);
        matching(guess, secret_number);
        if guess == secret_number{
            break;
        }
    }
    println!("Your Guesses are: {:?}", guesses);
    println!("Number of Guesses: {}", guesses.len())
}
