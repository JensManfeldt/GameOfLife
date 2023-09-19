use GameOfLife::game::{Game, new_game};
use yew::prelude::*;
use stylist::yew::{use_style};
use stylist::{Style, style, css};
use ndarray::ArrayBase;

struct GameWeb {
    cells: Game,
    rows: usize, 
    coloums: usize,
}

impl Component for GameWeb{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut game = Self {cells: new_game(10, 10), rows: 10, coloums: 10};
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





fn cells_matrix(matrix: &Game) -> Html {
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
        <div class={style}><p>{ format!("Cell Value {value:?}", value=cell_value) }</p></div>)

        
    }
    ).collect()
}


fn main() {
    yew::Renderer::<GameWeb>::new().render();
}