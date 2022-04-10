use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use connect4_lib::io::*;
use connect4_lib::play;

use ran::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct m {
    minmax_depth: isize,
    carlo_iter: isize,
}

pub fn my_evaluate_board(game: &mut Game, ai_conf: m) -> (isize, isize, ChipDescrip) {
    let is_max = game.get_turn() % 2 == 0;

    fn my_test_move(mov: isize, chip: ChipDescrip, game: &mut Game, ai_conf: m) -> isize {
        game.play(mov, chip);
        let mut score = my_minmax_search(game, ai_conf.minmax_depth) << (14 as isize);
        if score == 0 {
            score = my_monte_carlo_search(game, ai_conf);
        }
        game.undo_move();
        score
    }

    let mut potentials: Vec<(isize, isize, ChipDescrip)> = game
        .get_board()
        .get_valid_moves()
        .iter()
        .flat_map(|&mov| {
            game.current_player()
                .chip_options
                .iter()
                .map(move |&c| (mov, c))
        })
        .map(|(mov, c)| (my_test_move(mov, c, &mut game.clone(), ai_conf), mov, c))
        .collect();

    potentials.sort_by(|a, b| {
        if is_max {
            (b.0).partial_cmp(&a.0).unwrap()
        } else {
            (a.0).partial_cmp(&b.0).unwrap()
        }
    });

    // println!("{:?}", potentials);
    let (score, b_mov, c) = potentials[0];
    (score >> (14 as isize), b_mov, c)
}

fn my_monte_carlo_search(game: &mut Game, ai_conf: m) -> isize {
    let mut score = 0;
    let ri = Rnum::newi64();
    (0..ai_conf.carlo_iter).for_each(|_| {
        let mut moves = 0;
        let mut res = BoardState::Ongoing;
        let mut finished = false;
        while !finished {
            match res {
                BoardState::Ongoing => {
                    let m = game.get_board().get_valid_moves();
                    let lb = 0;
                    let up = m.len() as i64;
                    let r : usize = ran_irange(lb, up) as usize;
                    let mov = m[r];
                    let up1 = game.current_player().chip_options.len() as i64;
                    let r : usize = ran_irange(lb, up) as usize;
                    let chip = game.current_player().chip_options[r];
                    res = game.play(mov, chip);
                    moves += 1;
                }
                BoardState::Invalid => {
                    moves -= 1;
                    res = BoardState::Ongoing;
                }
                BoardState::Draw => {
                    finished = true;
                }
                BoardState::Win(x) => {
                    if x == 1 {
                        score += 1
                    } else {
                        score -= 1
                    }
                    finished = true;
                }
            }
        }
        for _ in 0..moves {
            game.undo_move()
        }
    });

    score
}

static mut COUNT: isize = 0;
// specifically a 2 player AI
// returns < 0 if player 2 wins
// returns > 0 if player 1 wins
fn my_minmax_search(game: &mut Game, depth: isize) -> isize {
    unsafe {
        COUNT += 1;
    }
    if depth == 0 {
        return 0;
    }

    let is_max = game.get_turn() % 2 == 0;
    if game.get_player(1).just_won(&game) {
        return -(depth as isize);
    }
    if game.get_player(0).just_won(&game) {
        return depth as isize;
    }

    let minmax: fn(isize, isize) -> isize = if is_max { std::cmp::max } else { std::cmp::min };

    let mut score = if is_max {
        std::isize::MIN
    } else {
        std::isize::MAX
    };

    let moves = game.get_board().get_valid_moves();
    let player = game.current_player().clone();
    for mov in moves {
        for chip in &player.chip_options {
            game.play_no_check(mov, *chip);
            score = minmax(score, my_minmax_search(game, depth - 1));
            game.undo_move();
        }
    }
    score
}

fn test() {
    // make a Game
    // for a game, we need a Board and a a Vec<player>
    // for Player, we need the PlayerType, Vec<chipDescrip>, Vec<Vec<chipDescrip>>
    let width = 7;
    let height = 6;

    let board = Board::new(width, height);
    let p1_chips = ChipDescrip {
        bg_color: 60,
        fg_color: 1,
        graphic: '◼',
    };
    let p2_chips = ChipDescrip {
        bg_color: 60,
        fg_color: 5,
        graphic: '◼',
    };

    let chip_options1: Vec<ChipDescrip> = vec![p1_chips];
    let win_cond1: Vec<Vec<ChipDescrip>> = vec![
        chip_options1.clone(),
        chip_options1.clone(),
        chip_options1.clone(),
        chip_options1.clone(),
    ];

    let chip_options2: Vec<ChipDescrip> = vec![p2_chips];
    let win_cond2: Vec<Vec<ChipDescrip>> = vec![
        chip_options2.clone(),
        chip_options2.clone(),
        chip_options2.clone(),
        chip_options2.clone(),
    ];
    let player1 = Player {
        player_type: PlayerType::Local,
        chip_options: chip_options1.clone(),
        win_conditions: win_cond1.clone(),
    };

    let player2 = Player {
        player_type: PlayerType::AI(HARD_AI),
        chip_options: chip_options2.clone(),
        win_conditions: win_cond2.clone(),
    };

    let mut my_game = Game::new(board, vec![player1, player2]);

    my_game.play(0, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(0, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(0, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(0, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(0, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(0, p2_chips);

    let (col1, chip1) = get_best_move(&mut my_game, HARD_AI);

    let a = m {
        carlo_iter: 4000,
        minmax_depth: 6,
    };
    let (_, col2, chip2) = my_evaluate_board(&mut my_game, a);

    println!("col = {}, chip = {:?}", col1, chip1);
    println!("col = {}, chip = {:?}", col2, chip2);
}

fn test2() {
    let board = Board::new(7, 6);
    let p1_chips = ChipDescrip {
        bg_color: 60,
        fg_color: 1,
        graphic: 'T',
    };

    let p2_chips = ChipDescrip {
        bg_color: 60,
        fg_color: 2,
        graphic: 'O',
    };

    let co1 = wrap_4_check(p1_chips, p2_chips);

    let co2 = wrap_4_check(p2_chips, p1_chips);
    let player1 = Player {
        player_type: PlayerType::Local,
        chip_options: co1.clone(),
        win_conditions: vec![co1.clone(), co1.clone(), co1.clone(), co1.clone()],
    };

    let player2 = Player {
        player_type: PlayerType::Local,
        chip_options: co2.clone(),
        win_conditions: vec![co2.clone(), co2.clone(), co2.clone(), co2.clone()],
    };

    let mut my_game = Game::new(board, vec![player1, player2]);

    my_game.play(0, p2_chips);
    my_game.play(0, p1_chips);
    my_game.play(0, p1_chips);
    my_game.play(4, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(3, p1_chips);
    my_game.play(4, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(6, p1_chips);
    my_game.play(4, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(2, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(2, p1_chips);
    my_game.play(1, p2_chips);
    my_game.play(4, p1_chips);
    my_game.play(2, p2_chips);

    let (col1, chip1) = get_best_move(&mut my_game, HARD_AI);

    let a = m {
        carlo_iter: 4000,
        minmax_depth: 6,
    };
    let (_, col2, chip2) = my_evaluate_board(&mut my_game, a);

    println!("col = {}, chip = {:?}", col1, chip1);
    println!("col = {}, chip = {:?}", col2, chip2);
    // let s = my_game.play(0, p2_chips);

    // match s {
    //     BoardState::Ongoing => {
    //     }
    //     BoardState::Invalid => {
    //     }
    //     BoardState::Draw => {
    //     }
    //     BoardState::Win(x) => {
    //         println!("Player {} has won", x);
    //     }
    // }
}
fn main() {
    test();
    //test2();

    let mut game = connect4_custom( PlayerType::Local,PlayerType::AI(MID_AI));

    //play(&mut game, TermIO::new());

    //println!("{}", game.get_board().get_valid_moves().len());
}
