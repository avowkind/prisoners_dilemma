extern crate prisoners_dilemma;
use prisoners_dilemma::*;
// use prisoners_dilemma::history::*;
// use prisoners_dilemma::choice::*;
use prisoners_dilemma::strategy::*;

const ROUNDS: usize = 10;

fn main() {
    println!("Starting Game");

    // put all the strategies into a list so we can try all the combinations.
    let strategies: Vec<&Strategy> =
        vec![
            &Never,
            &Always,
            &AlternateTrueFalse,
            &AlternateFalseTrue,
            &TitForTat
        ];

    for oppo in &strategies {
        for item in &strategies {
            let mut game = Game::new(ROUNDS);
            let score = game.play(*item, *oppo);
            println!("{} v {} {:?}", item.id(), oppo.id(), score);

        }
    }



}
