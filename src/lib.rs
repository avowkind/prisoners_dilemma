pub mod choice;
pub mod history;
pub mod strategy;

use choice::*;
use history::*;
use strategy::*;

/// Game holds the data for a sequence of encounters between two strategies.
pub struct Game {
    rounds: usize,
    h1: History,
    h2: History,
}

impl Game {

    pub fn new(rounds: usize) -> Self {
        Game {
            rounds: rounds,
            h1: History::new(rounds),
            h2: History::new(rounds),
        }
    }

    // play n rounds of PD between the two strategies, accumulate the history
    // and return the overall score.
    pub fn play<T, U>(&mut self, s1: &T, s2: &U) -> (u32, u32)
        where T: Strategy + ?Sized, U: Strategy + ?Sized
    {
        let mut score = (0,0);
        for _n in 0..self.rounds {
            let cp = ChoicePair(s1.choice(&self.h1), s2.choice(&self.h2));
            // print!("{:?}", cp);
            let res = cp.score();
            score = (score.0 + res.0, score.1 + res.1);
            // println!("{:?}", score);
            self.h2.push(cp.swap());
            self.h1.push(cp);
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game() {
        // create two histories and two strategies
        let mut game = Game::new(2);

        let s1 = AlternateTrueFalse;
        let s2 = TitForTat;
        let s3 = TitForTat;

        let score = game.play(&s1, &s2);
        println!("{:?}", score);

        let mut game2 = Game::new(2);
        let score = game2.play(&s2, &s3);
        println!("{:?}", score);
    }


}
