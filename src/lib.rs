pub struct Battleships {
    player_1: Board,
    player_2: Board,
    players_turn: Option<usize>,
    all_ships: Vec<usize>,
}

struct Board {
    ships_board: Vec<Vec<Cell>>,
    ships_left: u8,
    ships_placed: Vec<usize>,
    shots_board: Vec<Vec<Cell>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub contents: Option<Ship>,
    pub hit: bool,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ship {
    pub cells: Vec<Point>,
}

impl Battleships {
    pub fn new() -> Battleships {
        Battleships {
            player_1: Board {
                ships_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
                ships_left: 0,
                ships_placed: vec![],
                shots_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
            },
            player_2: Board {
                ships_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
                ships_left: 0,
                ships_placed: vec![],
                shots_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
            },
            players_turn: None,
            all_ships: vec![1, 2, 3, 4, 5],
        }
    }

    pub fn ships_board(&self, player: usize) -> Vec<Vec<Cell>> {
        match player {
            1 => self.player_1.ships_board.clone(),
            2 => self.player_2.ships_board.clone(),
            _ => panic!(),
        }
    }

    pub fn shots_board(&self, player: usize) -> Vec<Vec<Cell>> {
        match player {
            1 => self.player_1.shots_board.clone(),
            2 => self.player_2.shots_board.clone(),
            _ => panic!(),
        }
    }

    fn ships_left(&mut self, player: usize) -> &mut u8 {
        match player {
            1 => &mut self.player_1.ships_left,
            2 => &mut self.player_2.ships_left,
            _ => panic!(),
        }
    }

    pub fn place_ship(&mut self, player: usize, cells: Vec<Point>) -> Result<&str, &str> {
        match player {
            1 => {
                for cell in cells.clone() {
                    match self.player_1.ships_board[cell.x][cell.y].contents {
                        Some(_) => return Err("There's already a ship there."),
                        None => {},
                    }
                }
                for cell in cells.clone() {
                    self.player_1.ships_board[cell.x][cell.y] = Cell {
                        contents: Some(Ship { cells: cells.clone() }),
                        hit: false,
                    }
                }
                self.player_1.ships_left += 1;
                self.player_1.ships_placed.push(cells.len());
                return Ok("Placed ship.");
            }
            2 => {
                for cell in cells.clone() {
                    match self.player_2.ships_board[cell.x][cell.y].contents {
                        Some(_) => return Err("There's already a ship there."),
                        None => {},
                    }
                }
                for cell in cells.clone() {
                    self.player_2.ships_board[cell.x][cell.y] = Cell {
                        contents: Some(Ship { cells: cells.clone() }),
                        hit: false,
                    }
                }
                self.player_2.ships_left += 1;
                self.player_2.ships_placed.push(cells.len());
                return Ok("Placed ship.");
            }
            _ => panic!(),
        }
    }

    pub fn fire_at(&mut self, point: Point, player: usize) -> Result<&str, &str> {
        match self.players_turn {
            Some(turn) if player != turn => return Err("It's not your turn."),
            _ => {},
        }
        let other_player = match player {
            1 => 2,
            2 => 1,
            _ => panic!(),
        };
        let cell = self.get_cell(point, other_player);

        match cell.hit {
            true => Err("You've already fired at that cell."),
            false => {
                self.hit_cell(&point, other_player);
                match cell.contents {
                    Some(ref ship) => {
                        match self.sunk_ship(ship, other_player) {
                            true => {
                                let mut ships_left = self.ships_left(other_player);
                                *ships_left -= 1;
                                match *ships_left {
                                    0 => Ok("You sank all my battleships!"),
                                    _ => Ok("You sank my battleship!"),
                                }
                            }
                            false => {
                                Ok("Hit!")
                            },
                        }
                    }
                    None => {
                        self.players_turn = Some(other_player);
                        Ok("Miss.")
                    },
                }
            }
        }
    }

    fn hit_cell(&mut self, point: &Point, player: usize) {
        match player {
            1 => self.player_1.ships_board[point.x][point.y].hit = true,
            2 => self.player_2.ships_board[point.x][point.y].hit = true,
            _ => panic!(),
        }
    }

    fn get_cell(&self, point: Point, player: usize) -> Cell {
        match player {
            1 => self.player_1.ships_board[point.x][point.y].clone(),
            2 => self.player_2.ships_board[point.x][point.y].clone(),
            _ => panic!(),
        }
    }

    fn sunk_ship(&self, ship: &Ship, player: usize) -> bool {
        let ships_board = self.ships_board(player);
        for point in &ship.cells {
            if !ships_board[point.x][point.y].hit {
                return false;
            }
        }
        true
    }

    pub fn ships_in_place(&self, player: usize) -> bool {
        match player {
            1 => self.player_1.ships_placed == self.all_ships,
            2 => self.player_2.ships_placed == self.all_ships,
            _ => panic!(),
        }
    }
}