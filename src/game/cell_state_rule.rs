use super::game_of_life::{Game, Cell, NeighbourMatrix};
use ndarray::s;

pub trait CellStateRule: 'static {
    fn next_cell_state(&self, curr_game: &Game, cell: &Cell) -> Cell;
}

pub struct GameOfLifeRule;

impl CellStateRule for GameOfLifeRule {
    fn next_cell_state(&self, curr_game: &Game, cell: &Cell) -> Cell {
        let neightbours_matrix = get_neightbour_matrix(curr_game, (cell.x, cell.y));
        let living_neighbours = get_living_neightbours(&neightbours_matrix);

        let case1 = cell.value == 0 && living_neighbours == 3;
        let case2 = cell.value == 1 && (living_neighbours == 2 || living_neighbours == 3);

        let res = case1 || case2;
        if res {
            Cell {value: 1, x: cell.x, y: cell.y}
        }
        else {
            Cell {value: 0, x: cell.x, y: cell.y}
        }
    }
}

fn get_neightbour_matrix(game: &Game, cell: (usize, usize)) -> NeighbourMatrix {
    let neighbour_matrix = game.game_state.slice(s![cell.0 - 1 .. cell.0 + 2, cell.1 - 1 .. cell.1 + 2]);
    neighbour_matrix.to_owned()
}

fn get_living_neightbours(sub_matrix: &NeighbourMatrix) -> i32 {
    let mut neightbour_count = 0;
    for row in 0..3 {
        for col in 0..3 {
            if row == 1 && col == 1 {
                continue;
            }
            neightbour_count += sub_matrix[(row,col)].value ;
        }
    }
    neightbour_count
}