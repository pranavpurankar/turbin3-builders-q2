use crate::typestate::{Game, InProgress, Waiting};

pub trait CanSetup {
    fn setup(&self, game: Game<Waiting>, secret: u32) -> Game<InProgress>;
}

pub trait CanGuess {
    fn make_guess(&self, game: &mut Game<InProgress>, guess: u32) -> bool;
}

pub struct Dealer;
pub struct Player;

impl CanSetup for Dealer {
    fn setup(&self, game: Game<Waiting>, secret: u32) -> Game<InProgress> {
        println!("[Dealer] Secret number has been locked in!");
        game.start(secret)
    }
}

impl CanGuess for Player {
    fn make_guess(&self, game: &mut Game<InProgress>, guess: u32) -> bool {
        game.attempts += 1;
        if guess == game.secret {
            println!("[Player] Correct! It took {} tries.", game.attempts);
            true
        } else {
            let hint = if guess < game.secret { "Higher" } else { "Lower" };
            println!("[Player] Wrong! Hint: {}", hint);
            false
        }
    }
}