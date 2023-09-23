use GameOfLife::game::{Game, Cell, new_game, set_start, update, add_padding};
use yew::{prelude::*};
use stylist::{Style, style, css};


enum Msg {
   Step(i32),
   Resize(usize),
}
struct GameWeb
{
    cells: Game,
    rows: usize, 
    coloums: usize,
}

impl Component for GameWeb{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut fresh_game = new_game(10,10);
        set_start(&mut fresh_game);
        Self {cells: fresh_game, rows: 10, coloums: 10}
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_style = style!("place-items: center;").unwrap();
        let game_style = style!("display: inline-grid;border:1px solid black;").unwrap();
        html!(
            <div class={view_style}>
                <div class={game_style}>{
                    cells_matrix(&self.cells)
                }
                </div>
                <div>
                    <button onclick={ctx.link().callback(move |_| Msg::Step(1))}>
                        {"Step one"}
                    </button>
                    <button onclick={ctx.link().callback(move |_| Msg::Step(11))}>
                        {"Step 11"}
                    </button>
                    <button onclick={ctx.link().callback(move |_| Msg::Resize(1))}>
                        {"Resize 1"}
                    </button>
                </div>
            </div>
            )
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Step(num)=> {
                for it in 0..num{
                    self.cells = update(&self.cells)
                }
                true
            },
            Msg::Resize(num) => {
                self.cells = add_padding(&self.cells, num);
                self.coloums += num;
                self.rows += num;
                cells_matrix(&self.cells);
                true
            }
        }
    }

}



fn cells_matrix(game_state: &Game) -> Html {
    let mut cells = vec![];
    for i in 0..game_state.dim().0{
        for j in 0..game_state.dim().1{
                cells.push(game_state[(i,j)]);
            }
        };

    let mut row_index = 0;
    let mut column_index = 0;
    cells.iter().map(|cell_value| 
    { 
        if column_index == game_state.dim().0 {
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
                                        grid-column: ${grid_column};
                                        background-color: ${color};
                                        "#, 
                                        grid_row = row_index,
                                        grid_column = column_index,
                                        color = cell_to_color(cell_value).unwrap(),
                                        
                                    );

        let style = Style::new(style_string).unwrap(); 



        html!(
        <div class={style}><p>{ format!("Cell Value {value:?}", value=cell_value.value) }</p></div>
        )

    }
    ).collect()
}

fn cell_to_color(cell: &Cell) -> Result<String, String> {
    match cell.value {
        1 => Ok(String::from("#ffffff")),
        0 => Ok(String::from("#000000")),
        _ => Err(String::from("Cell has undefined value")),
    }
}


fn main() {
    yew::Renderer::<GameWeb>::new().render();
}