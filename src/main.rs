use nalgebra::{SMatrix, Matrix, ViewStorage,Dyn, Const};
use std::io::{stdout, Write};
use std::{thread, time};

extern crate log_update;
use log_update::LogUpdate;

type GameMatrix = SMatrix<i32, 10, 10>; 
//Don't understand lifetimes
//Also what does Dyn do, it just works with it here 
type NeighbourMatrix<'a> = Matrix<i32, Dyn,Dyn, ViewStorage<'a, i32,Dyn,Dyn,Const<1>, Const<10>>>; 
fn main() {

    let mut log_update = LogUpdate::new(stdout()).unwrap();
    let iteration = 100;

    let game_size = (10,10);
    let mut game = GameMatrix::zeros();

    set_start(&mut game);

    for i in 0..iteration{
        game = update(&game);

        log_update.render(&print_game(&game, game_size)).unwrap();

        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn set_start(game: &mut GameMatrix){
    game[(4,4)] = 1;
    game[(4,5)] = 1;
    game[(5,4)] = 1;

    game[(7,7)] = 1;
    game[(6,7)] = 1;
    game[(7,6)] = 1;
}

fn update(game: &GameMatrix) -> GameMatrix {
    let mut update_matrix = GameMatrix::zeros();

    for i in 1..9 {
        for j in 1..9 {
    
            let cell = (i as usize, j as usize);
            let neighbour_matrix = get_neightbour_matrix(game, cell);

            let living_neighbours: i32 = get_living_neightbours(&neighbour_matrix);

            let cell_state = next_cell_state(living_neighbours, game[cell]);

            if cell_state {
                update_matrix[cell] = 1;
            }
            else {
                update_matrix[cell] = 0;
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
            neightbour_count += sub_matrix[(row,col)] ;
        }
    }
    neightbour_count
}

fn get_neightbour_matrix(game: &GameMatrix, cell: (usize, usize)) -> NeighbourMatrix {
    let top= (cell.0 - 1, cell.1 - 1) ;
    let size = (3,3);
    let neighbour_matrix : NeighbourMatrix = game.view(top,size);
    neighbour_matrix
}

fn print_neighbour_matrix(matrix: &NeighbourMatrix, size: (usize,usize)){
    for i in 0..size.0{
        for j in 0..size.1{
            print!("{:?}", matrix[(i,j)]);
        }
        print!("\n");
    } 
}

fn print_game(game: &GameMatrix,size: (usize,usize)) -> String{
    let mut returnString = "".to_string();
    for i in 0..size.0{
        for j in 0..size.1{
            returnString += &format!("{}",game[(i,j)].to_string());
            //print!("{:?}", game[(i,j)]);
        }
        returnString += &format!("\n");
        //print!("\n");
    } 
    returnString
}