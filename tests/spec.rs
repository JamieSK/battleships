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
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 9, y: 9 }, 2).unwrap();
    assert_eq!(game.ships_board(1)[9][9], Cell { contents: None, hit: true });
}

#[test]
fn can_hit_cell_with_ship() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.ships_board(1)[1][1], Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: true });
}

#[test]
fn return_miss_when_hit_an_empty_cell() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    assert_eq!(game.fire_at(Point { x: 9, y: 9 }, 1), Ok("Miss."));
}

#[test]
fn return_hit_when_hit_a_ship() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    assert_eq!(game.fire_at(Point { x: 2, y: 1 }, 2), Ok("Hit!"));
}

#[test]
fn return_sank_my_battleship_when_all_of_ship_has_been_hit() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 2, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 2, y: 2 }, 2), Ok("You sank my battleship!"));
}

#[test]
fn hitting_one_players_ship_does_not_affect_the_others() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.ships_board(2)[1][1],
               Cell { contents: Some(Ship { cells: vec![Point { x: 1, y: 1 }] }), hit: false })
}

#[test]
fn return_sank_all_my_battleships_when_last_one_is_sunk() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 5, y: 1 }, 2).unwrap();
    game.fire_at(Point { x: 5, y: 2 }, 2).unwrap();
    game.fire_at(Point { x: 5, y: 3 }, 2).unwrap();
    game.fire_at(Point { x: 5, y: 4 }, 2).unwrap();
    game.fire_at(Point { x: 5, y: 5 }, 2).unwrap();
    game.fire_at(Point { x: 4, y: 1 }, 2).unwrap();
    game.fire_at(Point { x: 4, y: 2 }, 2).unwrap();
    game.fire_at(Point { x: 4, y: 3 }, 2).unwrap();
    game.fire_at(Point { x: 4, y: 4 }, 2).unwrap();
    game.fire_at(Point { x: 3, y: 1 }, 2).unwrap();
    game.fire_at(Point { x: 3, y: 2 }, 2).unwrap();
    game.fire_at(Point { x: 3, y: 3 }, 2).unwrap();
    game.fire_at(Point { x: 2, y: 1 }, 2).unwrap();
    game.fire_at(Point { x: 2, y: 2 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 2), Ok("You sank all my battleships!"));
}

#[test]
fn can_sink_player_twos_battleships() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 5, y: 1 }, 1).unwrap();
    game.fire_at(Point { x: 5, y: 2 }, 1).unwrap();
    game.fire_at(Point { x: 5, y: 3 }, 1).unwrap();
    game.fire_at(Point { x: 5, y: 4 }, 1).unwrap();
    game.fire_at(Point { x: 5, y: 5 }, 1).unwrap();
    game.fire_at(Point { x: 4, y: 1 }, 1).unwrap();
    game.fire_at(Point { x: 4, y: 2 }, 1).unwrap();
    game.fire_at(Point { x: 4, y: 3 }, 1).unwrap();
    game.fire_at(Point { x: 4, y: 4 }, 1).unwrap();
    game.fire_at(Point { x: 3, y: 1 }, 1).unwrap();
    game.fire_at(Point { x: 3, y: 2 }, 1).unwrap();
    game.fire_at(Point { x: 3, y: 3 }, 1).unwrap();
    game.fire_at(Point { x: 2, y: 1 }, 1).unwrap();
    game.fire_at(Point { x: 2, y: 2 }, 1).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 1), Ok("You sank all my battleships!"));
}

#[test]
fn cannot_play_on_same_cell_twice() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 1, y: 1 }, 1).unwrap();
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 1), Err("You've already fired at that cell."));
}

#[test]
fn cannot_place_ship_where_another_is() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]).unwrap();
    assert_eq!(game.place_ship(1, vec![Point { x: 1, y: 1 }]), Err("There's already a ship there."));
}

#[test]
fn cannot_play_twice_in_a_row_as_the_same_player() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 1);
    place_all_ships(&mut game, 2);
    game.fire_at(Point { x: 9, y: 9 }, 1).unwrap();
    assert_eq!(game.fire_at(Point { x: 2, y: 2 }, 1), Err("It's not your turn."));
}

#[test]
fn can_play_again_if_you_hit_a_ship() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 2);
    place_all_ships(&mut game, 1);
    game.fire_at(Point { x: 1, y: 1 }, 2).unwrap();
    assert_eq!(game.fire_at(Point { x: 1, y: 2 }, 2), Ok("Miss."));
}

#[test]
fn starts_with_ships_not_placed() {
    let mut game = Battleships::new();
    assert_eq!(game.ships_in_place(1), false);
}

#[test]
fn returns_true_once_all_ships_are_positioned() {
    let mut game = Battleships::new();
    place_all_ships(&mut game, 1);
    assert_eq!(game.ships_in_place(1), true);
}

#[test]
fn can_place_ships_in_any_order() {
    let mut game = Battleships::new();
    game.place_ship(1, vec![Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 5, y: 3 }, Point { x: 5, y: 4 }, Point { x: 5, y: 5 }]).unwrap();
    game.place_ship(1, vec![Point { x: 2, y: 1 }, Point { x: 2, y: 2 }]).unwrap();
    game.place_ship(1, vec![Point { x: 4, y: 1 }, Point { x: 4, y: 2 }, Point { x: 4, y: 3 }, Point { x: 4, y: 4 }]).unwrap();
    game.place_ship(1, vec![Point { x: 1, y: 1 }]).unwrap();
    game.place_ship(1, vec![Point { x: 3, y: 1 }, Point { x: 3, y: 2 }, Point { x: 3, y: 3 }]).unwrap();
    assert_eq!(game.ships_in_place(1), true);
}

#[test]
fn cannot_play_until_all_ships_are_placed() {
    let mut game = Battleships::new();
    assert_eq!(game.fire_at(Point { x: 1, y: 1 }, 1), Err("Not all ships have been placed."))
}

fn place_all_ships(game: &mut Battleships, player: usize) {
    game.place_ship(player, vec![Point { x: 1, y: 1 }]).unwrap();
    game.place_ship(player, vec![Point { x: 2, y: 1 }, Point { x: 2, y: 2 }]).unwrap();
    game.place_ship(player, vec![Point { x: 3, y: 1 }, Point { x: 3, y: 2 }, Point { x: 3, y: 3 }]).unwrap();
    game.place_ship(player, vec![Point { x: 4, y: 1 }, Point { x: 4, y: 2 }, Point { x: 4, y: 3 }, Point { x: 4, y: 4 }]).unwrap();
    game.place_ship(player, vec![Point { x: 5, y: 1 }, Point { x: 5, y: 2 }, Point { x: 5, y: 3 }, Point { x: 5, y: 4 }, Point { x: 5, y: 5 }]).unwrap();
}