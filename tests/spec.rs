extern crate battleships;

use battleships::*;

#[test]
fn player_1_starts_with_a_blank_ships_board() {
    let game = Battleships::new();
    assert_eq!(game.ships_board(1), vec![vec![Cell { contents: None, hit: false }; 10]; 10]);
}

#[test]
fn player_1_starts_with_a_blank_shots_board() {
    let game = Battleships::new();
    assert_eq!(game.shots_board(1), vec![vec![Cell { contents: None, hit: false }; 10]; 10]);
}

#[test]
fn can_place_a_submarine_on_the_board() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]);
    assert_eq!(game.ships_board(1)[1][1],
               Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: false });
}

#[test]
fn ship_on_board_starts_not_hit() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]);
    assert_eq!(game.ships_board(1)[1][1],
               Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: false });
}

#[test]
fn can_hit_empty_cell() {
    let mut game = Battleships::new();
    game.fire_at(Point { x: 1, y: 1 }, 1);
    assert_eq!(game.ships_board(2)[1][1], Cell { contents: None, hit: true });
}