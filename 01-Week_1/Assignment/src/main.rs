mod typestate;
mod capabilities;

use crate::capabilities::{CanGuess, CanSetup, Dealer, Player};
use crate::typestate::Game;
use std::io::{self, Write};

fn main() {
    let game = Game::new();
    let dealer = Dealer;
    let player = Player;

    println!("--- Welcome to the RBAC Guessing Game ---");

    // 1. Dealer Setup
    print!("Dealer, enter secret number: ");
    io::stdout().flush().unwrap();
    let mut secret = String::new();
    io::stdin().read_line(&mut secret).unwrap();
    let secret: u32 = secret.trim().parse().expect("Please type a number!");

    let mut active_game = dealer.setup(game, secret);

    // 2. Clear screen so player doesn't see the number
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // 3. Player Loop
    loop {
        print!("Player, take a guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if player.make_guess(&mut active_game, guess) {
            let _finished_game = active_game.end();
            println!("Game Over! Thanks for playing.");
            break;
        }
    }
}