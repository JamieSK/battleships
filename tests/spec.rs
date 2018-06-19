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
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.ships_board(1)[1][1], Cell { contents: None, hit: true });
}

#[test]
fn can_hit_cell_with_ship() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]);
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.ships_board(1)[1][1], Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: true });
}

#[test]
fn return_miss_when_hit_an_empty_cell() {
    let mut game = Battleships::new();
    assert_eq!(game.fire_at(Point { x: 5, y: 5 }, 1), Ok("Miss."));
}

#[test]
fn return_hit_when_hit_a_ship() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]);
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 2), Ok("Hit!"));
}

#[test]
fn return_sunk_my_battleship_when_all_of_ship_has_been_hit() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]);
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 2 }, 2), Ok("You sank my battleship!"));
}