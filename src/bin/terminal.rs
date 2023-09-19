use log_update::LogUpdate;
use std::{thread, time};
use std::io::{stdout};

use GameOfLife::game::*;
fn main() {

    let mut log_update = LogUpdate::new(stdout()).unwrap();
    let iteration = 100;

    let game_size = (10,10);
    let mut game = new_game(10, 10);

    set_start(&mut game);

    for _i in 0..iteration{
        game = update(&game);

        log_update.render(&print_game(&game, game_size)).unwrap();

        thread::sleep(time::Duration::from_millis(1000));
        log_update.clear().unwrap();
    }
}