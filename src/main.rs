use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    println!("Rock-Paper-Scissor Game");
    let mut game = true;

    while game {
        println!("Enter (r)ock (p)aper or (s)cissor or (q)uit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!("You played: {}", choice);

        let choices = [String::from("r"), String::from("p"), String::from("s")];
        let mut rng = thread_rng();
        let comp_choice = choices.choose(&mut rng).expect("Empty Range");

        println!("Computer played: {}", comp_choice);

        if choice.trim() == comp_choice {
            println!("You won, hehe");
        } else if choice.trim() == "q" {
            game = false;
        } else if choice.trim() == "r" && comp_choice == "s" {
            println!("You won, hehe");
        } else if choice.trim() == "p" && comp_choice == "r" {
            println!("You won, hehe");
        } else if choice.trim() == "s" && comp_choice == "p" {
            println!("You won, hehe");
        } else {
            println!("You lost, hehe");
        }
    }
}
