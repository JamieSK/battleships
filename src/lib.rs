pub struct Battleships {}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {}

pub struct Point {}

pub struct Ship {}

impl Battleships {
    pub fn new() -> Battleships {
        Battleships {}
    }

    pub fn ships_board(&self, _player: usize) -> Vec<Vec<Cell>> {
        vec![vec![Cell::new(None, false); 10]; 10]
    }

    pub fn shots_board(&self, _player: usize) -> Vec<Vec<Cell>> {
        vec![vec![Cell::new(None, false); 10]; 10]
    }

    pub fn place_ship(&mut self, _player: usize, _cells: Vec<Point>) {}
}

impl Cell {
    pub fn new(_contents: Option<Ship>, _hit: bool) -> Cell {
        Cell {}
    }
}

impl Point {
    pub fn new(_x: usize, _y:usize) -> Point {
        Point {}
    }
}

impl Ship {
    pub fn new(_cells: Vec<Point>) -> Ship {
        Ship {}
    }
}