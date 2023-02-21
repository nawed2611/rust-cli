use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    let mut choice = String::new();
    let mut rng = thread_rng();
    let choices = [String::from("r"), String::from("p"), String::from("s")];
    let comp_choice = choices.choose(&mut rng).expect("Failed to choose");

    println!("Rock-Paper-Scissor Game");
    println!("Enter (r)ock (p)aper or (s)cissor");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("You chose {}", choice);
    println!("Computer chose {}", comp_choice);

    if &choice == comp_choice {
        println!("It's a tie!");
    } else if choice == "r" && comp_choice == "s" {
        println!("You win!");
    } else if choice == "p" && comp_choice == "r" {
        println!("You win!");
    } else if choice == "s" && comp_choice == "p" {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
