extern crate battleships;

use battleships::*;

#[test]
fn player_1_starts_with_a_blank_ships_board() {
    let game = Battleships::new();
    assert_eq!(game.ships_board(1), vec![vec![Cell::new(None, false); 10]; 10]);
}

#[test]
fn player_1_starts_with_a_blank_shots_board() {
    let game = Battleships::new();
    assert_eq!(game.shots_board(1), vec![vec![Cell::new(None, false); 10]; 10]);
}

#[test]
fn can_place_a_submarine_on_the_board() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point::new(1, 1)]);
    assert_eq!(game.ships_board(1)[2][2],
               Cell::new(Some(Ship::new(vec![Point::new(1, 1)])), false));
}

#[test]
fn ship_on_board_starts_not_hit() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point::new(1, 1)]);
    assert_eq!(game.ships_board(1)[2][2],
               Cell::new(Some(Ship::new(vec![Point::new(1, 1)])), false));
}