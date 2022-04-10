use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use connect4_lib::io::*;
use connect4_lib::play;

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

impl PlayerTurn {
    pub fn flip(mut self) -> Self{
        match self {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        }
    }
}

fn find_i(state: &mut Vec<char>, col: usize, w: usize, index: usize, player : char) -> i32 {
    if index >= state.len() {
        return -1;
    }
    let my_col = (index + 1) % w;
    //last index before going to the next row
    if my_col == col {
        if state[index] == '0' {
            if find_i(state, col, w, index + w,player) == -1 {
                state[index] = player;
                return 0;
            } else {
                return 0;
            }
        } else {
            //change the pervious state of it
            state[index - w] = '5';
            return 0;
        }
    } else {
        find_i(state, col, w, index + 1,player)
    }
}

fn run() {
    let mut mg = connect4_custom(PlayerType::AI(HARD_AI), PlayerType::AI(HARD_AI));
    let x = mg.current_player().chip_options.clone()[0];
    let y = mg.next_player().chip_options.clone()[0];

    println!("{:?}", x);
    println!("{:?}", y);
    // let h = mg.get_board().height();
    // let w = mg.get_board().width();

    // let mut g = "1".to_string();
    // for _ in 0..41 {
    //     g += "0";
    // }

    // let col = 2;
    // let mut j = 0;
    // let mut s: Vec<char> = g.chars().collect();

    // s[36] = '4';
    // println!("h = {}, w = {}, chars_len = {}", h, w, g.len());

    // find_i(&mut s, col, w, 0);

    // for i in 0..s.len() {
    //     println!("{} - {}", i, s[i]);
    // }
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
}

fn main() {
    //run();

    //test();
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
        win_conditions: vec![
            co1.clone(),
            co1.clone(),
            co1.clone(),
            co1.clone(),
        ],
    };

    let player2 = Player {
        player_type: PlayerType::Local,
        chip_options: co2.clone(),
        win_conditions: vec![
            co2.clone(),
            co2.clone(),
            co2.clone(),
            co2.clone(),
        ],
    };

    let mut my_game = Game::new(board, vec![player1, player2]);

    my_game.play(0, p2_chips);
    my_game.play(0, p1_chips);
    my_game.play(0, p1_chips);
    let s = my_game.play(0, p2_chips);

    match s {
        BoardState::Ongoing => {
    
        }
        BoardState::Invalid => {
        }
        BoardState::Draw => {
        }
        BoardState::Win(x) => {
            println!("Player {} has won", x);
        }
    }
}
