use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use connect4_lib::io::*;
use connect4_lib::play;


fn run() {
    let mut mg = connect4_custom(PlayerType::Local,PlayerType::AI(EASY_AI));
    play(&mut mg, TermIO::new());
}

fn main() {
    run();
}