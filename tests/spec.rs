extern crate battleships;

use battleships::*;

#[test]
fn player_1_starts_with_a_blank_ships_board() {
    let game = Battleships::new();
    assert_eq!(game.ships_board(1), vec![vec![None; 10]; 10]);
}

#[test]
fn player_1_starts_with_a_blank_shots_board() {
    let game = Battleships::new();
    assert_eq!(game.shots_board(1), vec![vec![None; 10]; 10]);
}