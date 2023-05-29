use GameOfLife::game::{self, GameMatrix};
use yew::prelude::*;
use stylist::yew::{use_style};
use stylist::{Style, style, css};


struct Game {
    cells: GameMatrix,
    rows: usize, 
    coloums: usize,
}

impl Component for Game{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut game = Self {cells: GameMatrix::zeros(), rows:10, coloums:10};
        for i in 0..game.rows{
            for j in 0..game.coloums{
                game.cells[(i,j)] = 0;
            }
        }
        game 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = style!("display: inline-grid;border:1px solid black;").unwrap();
        html!(
                <div class={style}>{
                    cells_matrix(&self.cells)
                }
                </div>
            )
            }
    }



fn cells_matrix(matrix: &GameMatrix) -> Html {
    let mut cells = vec![];
    for i in 0..10 {
        for j in 0..10 {
                cells.push(matrix[(i,j)]);
            }
        };

    let mut row_index = 0;
    let mut column_index = 0;
    cells.iter().map(|cell_value| 
    { 
        if column_index == 10 {
            column_index = 1;
            row_index += 1;
        }
        else {
            column_index += 1;
        }

        let style_string = css!(r#"border:1px solid black;
                                        height: 100px;
                                        width: 100px;
                                        grid-row: ${grid_row};
                                        grid-column: ${grid_column};"#, 
                                        grid_row = row_index,
                                        grid_column = column_index,
                                    );

        let style = Style::new(style_string).unwrap(); 



        html!(
        <div class={style}><p>{ format!("Cell Value {value}", value=cell_value) }</p></div>)

        
    }
    ).collect()
}


#[derive(Debug, PartialEq, Clone)]
struct Cell{
    value: i32,
}

fn main() {
    yew::Renderer::<Game>::new().render();
}