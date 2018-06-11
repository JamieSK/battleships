pub struct Battleships {}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {}

impl Battleships {
    pub fn new() -> Battleships {
        Battleships {}
    }

    pub fn ships_board(&self, _player: usize) -> Vec<Vec<Option<Cell>>> {
        vec![vec![None; 10]; 10]
    }
}