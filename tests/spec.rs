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
    game.place_ship(1, vec![Point { x: 1, y: 1 }]).unwrap();
    assert_eq!(game.ships_board(1)[1][1],
               Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: false });
}

#[test]
fn ship_on_board_starts_not_hit() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]).unwrap();
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
    game.place_ship(1, vec![Point { x: 1, y: 1 }]).unwrap();
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
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 2), Ok("Hit!"));
}

#[test]
fn return_sank_my_battleship_when_all_of_ship_has_been_hit() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    game.place_ship(1, vec![Point { x: 4, y: 4 }]).unwrap();
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 2 }, 2), Ok("You sank my battleship!"));
}

#[test]
fn hitting_one_players_ship_does_not_affect_the_others() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    game.place_ship(2, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.ships_board(2)[1][1],
               Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }] }), hit: false })
}

#[test]
fn return_sank_all_my_battleships_when_last_one_is_sunk() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 2 }, 2), Ok("You sank all my battleships!"));
}

#[test]
fn can_sink_player_twos_battleships() {
    let mut game = Battleships::new();
    game.place_ship(2, vec![Point { x: 1, y: 1 }, Point { x: 1, y: 2 }]).unwrap();
    game.fire_at(Point { x: 1, y: 1 }, 1).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 2 }, 1), Ok("You sank all my battleships!"));
}

#[test]
fn cannot_play_on_same_cell_twice() {
    let mut game = Battleships::new();
    game.fire_at(Point { x: 1, y: 1 }, 1).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 1), Err("You've already fired at that cell."));
}

#[test]
fn cannot_place_ship_where_another_is() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1}]).unwrap();
    assert_eq!(game.place_ship(1, vec![Point { x: 1, y: 1}]), Err("There's already a ship there."));
}