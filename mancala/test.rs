use super::*;

fn best(moves: &[Pit]) -> Pit {
    let game = Mancala::load(&moves);
    let t = Duration::new(1,0);
    let result = MCTS::new().with_time(t).search(game);
    println!("{:?}",result);
    result
}

#[test]
fn mancala_free_move_1() {
    let mut game = Mancala::new();
    println!("{}",game);

    game = game.make(Pit::R3);
    println!("{}",game);

    game = game.make(Pit::R6);
    println!("{}",game);

    assert!(game.side == Player::L);
    assert!(game.pit[RB] == 2);
    assert!(game.pit[LB] == 0);
}

#[test]
fn mancala_free_move_2() {
    let mut game = Mancala::new();
    println!("{}",game);
    
    game = game.make(Pit::R3);
    println!("{}",game);

    game = game.make(Pit::R6);
    println!("{}",game);

    game = game.make(Pit::L2);
    println!("{}",game);

    game = game.make(Pit::L6);
    println!("{}",game);


    assert!(game.side == Player::R);
    assert!(game.pit[RB] == 2);
    assert!(game.pit[LB] == 2);
}

#[test]
fn mancala_right_capture() {
    let mut game = Mancala::new();
    println!("{}",game);

    game = game.make(Pit::R6);
    println!("{}",game);
    
    game = game.make(Pit::L6);
    println!("{}",game);
    
    game = game.make(Pit::R1);
    println!("{}",game);
    
    assert!(game.side == Player::L);
    assert!(game.pit[RB] == 1 + 1 + 5);
    assert!(game.pit[LB] == 1);
    assert!(game.pit[Pit::R6 as usize] == 0);
    assert!(game.pit[Pit::L1 as usize] == 0);
}

#[test]
fn mancala_left_capture() {
    let mut game = Mancala::new();
    println!("{}",game);

    game = game.make(Pit::R6);
    println!("{}",game);
    
    game = game.make(Pit::L2);
    println!("{}",game);
    
    game = game.make(Pit::L6);
    println!("{}",game);
    
    game = game.make(Pit::R5);
    println!("{}",game);
    
    game = game.make(Pit::L5);
    println!("{}",game);
    
    game = game.make(Pit::R2);
    println!("{}",game);
    
    game = game.make(Pit::L3);
    println!("{}",game);
    
    game = game.make(Pit::R1);
    println!("{}",game);
    
    game = game.make(Pit::L2);
    println!("{}",game);
    
    assert!(game.side == Player::R);
    assert!(game.pit[RB] == 4);
    assert!(game.pit[LB] == 12);
    assert!(game.pit[Pit::R4 as usize] == 0);
    assert!(game.pit[Pit::L3 as usize] == 0);
}

#[test]
fn mancala_best_move_split() {
    let m = best(&[R6,L6]);
    assert!((m == R1) || (m == R2));
}

#[test]
fn mancala_best_move_free_turn() {
    let m = best(&[R6,L6,R2]);
    assert!(m == R6);
}
