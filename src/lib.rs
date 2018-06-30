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

    fn ships_left(&mut self, player: usize) -> &mut u8 {
        match player {
            1 => &mut self.player_1.ships_left,
            2 => &mut self.player_2.ships_left,
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

    pub fn fire_at(&mut self, point: Point, player: usize) -> Result<&str, &str> {
        let other_player = match player {
            1 => 2,
            2 => 1,
            _ => panic!(),
        };
        self.hit_cell(&point, other_player);
        let cell = self.get_cell(point, other_player);
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
                    },
                    false => Ok("Hit!"),
                }
            }
            None => Ok("Miss."),
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
}