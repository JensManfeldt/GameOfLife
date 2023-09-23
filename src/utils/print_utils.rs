use crate::game::game_of_life::{Game, NeighbourMatrix, Cell};

pub fn print_neighbour_matrix(matrix: &NeighbourMatrix, size: (usize,usize)){
    for i in 0..size.0{
        for j in 0..size.1{
            print!("{:?}", matrix[(i,j)].value);
        }
        print!("\n");
    } 
}

pub fn print_game(game: &Game ,size: (usize,usize)) -> String{
    let mut return_string = "".to_string();
    for i in 0..size.0{
        for j in 0..size.1{
            return_string += &game[(i,j)].value.to_string();
        }
        return_string += &'\n'.to_string();
    } 
    return_string
}