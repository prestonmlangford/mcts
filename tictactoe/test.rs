use super::*;
use mcts::Search as Search;
use std::time::Duration;

fn best(moves: &[Move]) -> Move {
    let game = StateManager::load(&moves);
    let mut search = Search::new(game).with_time(Duration::new(1, 0));
    let result = search.execute();
    println!("{:?}",result);
    result
}

#[test]
fn tictactoe_best_obvious() {
    assert!(best(&[MM,TM,MR,ML,BR,TR]) == TL);
}

#[test]
fn tictactoe_best_even() {
    assert!(best(&[TL,MM,ML]) == BL);
}

#[test]
fn tictactoe_best_even2() {
    assert!(best(&[MM,ML,MR,TL]) == BL);
}

#[test]
fn tictactoe_best_split() {
    let m = best(&[MM,TM,MR,ML]);
    assert!((m == BR) || (m == TR));
}