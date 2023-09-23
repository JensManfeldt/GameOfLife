use ndarray::Array2;
use ndarray::s;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameCell {
    pub value: i32,
}

impl GameCell {
    fn new() -> GameCell {
        GameCell {value : 0}
    }
}

pub type Cell = GameCell;
pub type NeighbourMatrix = Array2<Cell>;
pub type Game = Array2<Cell>;

pub fn new_game(height: usize, width: usize) -> Game {
    Game::from_shape_fn((height, width), |(_i,_j)| Cell::new() )
}

pub fn set_start(game: &mut Game){
    game[(4,4)].value = 1;
    game[(4,5)].value = 1;
    game[(5,4)].value = 1;

    game[(7,7)].value = 1;
    game[(6,7)].value = 1;
    game[(7,6)].value = 1;
}

pub fn update(game: &Game) -> Game {
    let curr_size = game.dim();
    let mut update_matrix = new_game(curr_size.0, curr_size.1);

    for i in 1..curr_size.0-1 {
        for j in 1..curr_size.1-1 {
    
            let cell = (i as usize, j as usize);
            let neighbour_matrix = get_neightbour_matrix(game, cell);

            let living_neighbours: i32 = get_living_neightbours(&neighbour_matrix);

            let cell_state = next_cell_state(living_neighbours, game[cell].value);

            if cell_state {
                update_matrix[cell].value = 1;
            }
            else {
                update_matrix[cell].value = 0;
            }

        }
    }
    update_matrix
}

//Kind of weird that this returns a bool since the cell have states that is ints
//An Enum might be more propper
fn next_cell_state(living_neighbours: i32, curr_cell_state: i32) -> bool {
    let case1 = curr_cell_state == 0 && living_neighbours == 3;
    let case2 = curr_cell_state == 1 && (living_neighbours == 2 || living_neighbours == 3);

    case1 || case2
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

pub fn add_padding(game: &Game, padding: usize) -> Game {
    let curr_size = game.dim();
    let mut new_game = new_game(curr_size.0 + (2 * padding) , curr_size.1 + (2 * padding));
    println!("New game dim {:?}", new_game.dim());
    for i in padding..curr_size.0 {
        for j in padding..curr_size.1 {
            new_game[(i,j)] = game[(i,j)];
        }
    }

    new_game
}

pub fn get_neightbour_matrix(game: &Game, cell: (usize, usize)) -> NeighbourMatrix {
    let neighbour_matrix = game.slice(s![cell.0 - 1 .. cell.0 + 2, cell.1 - 1 .. cell.1 + 2]);
    neighbour_matrix.to_owned()
}
