use ndarray::Array2;
use ndarray::s;
use super::cell_state_rule::{CellStateRule, GameOfLifeRule};

pub type Cell = GameCell;
pub type NeighbourMatrix = Array2<Cell>;
pub type GameRepresentation = Array2<Cell>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameCell {
    pub value: i32,
    pub x: usize, 
    pub y: usize,
}

impl GameCell {
    fn new(value: i32, x: usize, y: usize) -> GameCell {
        GameCell {value, x, y}
    }
}

pub struct Game {
    pub game_state: GameRepresentation,
    pub cell_state_rule: Box<dyn CellStateRule>,
}

impl Game {
    pub fn get_game_size(&self) -> (usize, usize) {
        self.game_state.dim()
    }
}

pub fn new_game(height: usize, width: usize) -> Game {
    let game_state = GameRepresentation::from_shape_fn((height, width), |(i,j)| Cell::new(0, i, j));
    Game{game_state, cell_state_rule: Box::new(GameOfLifeRule)}
}

pub fn set_start(game: &mut Game){
    game.game_state[(4,4)] = Cell::new(1,4,4);
    game.game_state[(4,5)] = Cell::new(1,4,5);
    game.game_state[(5,4)] = Cell::new(1,5,4);

    game.game_state[(7,7)] = Cell::new(1,7,7);
    game.game_state[(6,7)] = Cell::new(1,6,7);
    game.game_state[(7,6)] = Cell::new(1,7,6);
}

pub fn update(game: &Game) -> Game {
    let curr_size = game.get_game_size();
    let mut update_matrix = new_game(curr_size.0, curr_size.1);

    let updatedable_game = game.game_state.slice(s![1 .. curr_size.0 - 1, 1 .. curr_size.1 - 1]);
    
    for cell in updatedable_game.iter(){
        update_matrix.game_state[(cell.x, cell.y)] = game.cell_state_rule.next_cell_state(game, cell);
    }

    update_matrix
}

pub fn add_padding(game: &Game, padding: usize) -> Game {
    let curr_size = game.get_game_size();
    let mut new_game = new_game(curr_size.0 + (2 * padding) , curr_size.1 + (2 * padding));
    for i in padding..curr_size.0 {
        for j in padding..curr_size.1 {
            new_game.game_state[(i,j)] = game.game_state[(i,j)];
        }
    }

    new_game
}
