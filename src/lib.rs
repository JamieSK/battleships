pub struct Battleships {
    player_1: Board,
    player_2: Board,
}

struct Board {
    ships_board: Vec<Vec<Cell>>,
    ships_left: u8,
    shots_board: Vec<Vec<Cell>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub contents: Option<Ship>,
    pub hit: bool,
}

#[derive(Clone, PartialEq, Debug)]
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
                shots_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
            },
            player_2: Board {
                ships_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
                ships_left: 0,
                shots_board: vec![vec![Cell { contents: None, hit: false }; 10]; 10],
            },
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

    pub fn place_ship(&mut self, player: usize, cells: Vec<Point>) {
        match player {
            1 => {
                for cell in cells.clone() {
                    self.player_1.ships_board[cell.x][cell.y] = Cell {
                        contents: Some(Ship { cells: cells.clone() }),
                        hit: false,
                    }
                }
                self.player_1.ships_left += 1;
            }
            2 => {
                for cell in cells.clone() {
                    self.player_2.ships_board[cell.x][cell.y] = Cell {
                        contents: Some(Ship { cells: cells.clone() }),
                        hit: false,
                    }
                }
                self.player_2.ships_left += 1;
            }
            _ => panic!(),
        }
    }

    pub fn fire_at(&mut self, point: Point, _player: usize) -> Result<&str, &str> {
        self.player_1.ships_board[point.x][point.y].hit = true;
        let cell = &self.player_1.ships_board[point.x][point.y];
        match cell.contents {
            Some(ref ship) => {
                match self.sunk_ship(ship, 1) {
                    true => {
                        self.player_1.ships_left -= 1;
                        match self.player_1.ships_left {
                            0 => Ok("You sank all my battleships!"),
                            _ => Ok("You sank my battleship!"),
                        }
                    },
                    false => Ok("Hit!"),
                }
            }
            None => Ok("Miss."),
        }
    }

    fn sunk_ship(&self, ship: &Ship, player: usize) -> bool {
        let mut sunk = true;
        let ships_board = self.ships_board(player);
        for point in &ship.cells {
            if !ships_board[point.x][point.y].hit {
                sunk = false;
                break;
            }
        }
        sunk
    }
}