use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use connect4_lib::io::*;
use connect4_lib::play;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

impl PlayerTurn {
    pub fn flip(mut self) -> Self {
        match self {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        }
    }
}

fn find(state : String, col : usize, w: usize, h: usize, player : &str) -> String {
    let mut statecpy = state.clone();
    let temp = statecpy.as_bytes();
    let mut v : Vec<u8> = vec![];
    for i in 0..h {
        let ind = i * w + col;
        v.push(temp[ind]);
    }

    let mut index = h + 1;

    for i in (0..v.len()).rev() {
        if v[i] == 111 { //byte for o
            //first index to have o in it, so the bottom
            index = i;
            break;
        }
    }

    if h + 1 == index {
        //no empty spot in this column
        //internal game should take care of this
        println!("Saved");
        statecpy
    } else {
        let ind = index * w + col;
        statecpy.replace_range(ind..ind + 1, &player[..]);
        statecpy
    }
}

fn print_state(gameState: &String, height: usize, width: usize) {
    let mut state_holder = "\n".to_string();
    for i in 0..height {
        let start = width * i;
        let end = width * (i + 1) - 1;
        let holder = &gameState[start..end];
        state_holder += &gameState[start..=end];
        state_holder += "\n";
    }
    println!("{}", state_holder);
}

fn run() {
    let mut mg = connect4_custom(PlayerType::AI(HARD_AI), PlayerType::AI(HARD_AI));
    let x = mg.current_player().chip_options.clone()[0];
    let y = mg.next_player().chip_options.clone()[0];

    let h = mg.get_board().height();
    let w = mg.get_board().width();

    let mut g = "o".to_string();
    for _ in 0..41 {
        g += "o";
    }

    let col = 2;
    let mut j = 0;
    let mut s: Vec<char> = g.chars().collect();


    let g = find(g, col, w, h, "1");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "2");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "1");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "2");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "1");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "2");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "1");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "2");
    print_state(&g, h, w);
    let g = find(g, col, w, h, "1");
    print_state(&g, h, w);
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

    let chip_options1: Vec<ChipDescrip> = four_in_a_row(p1_chips);
    let win_cond1: Vec<Vec<ChipDescrip>> = vec![
        chip_options1.clone(),
        chip_options1.clone(),
        chip_options1.clone(),
        chip_options1.clone(),
    ];

    let chip_options2: Vec<ChipDescrip> = four_in_a_row(p2_chips);
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

    //let mut my_game = connect4_custom(PlayerType::Local, PlayerType::Local);

    let c = my_game.current_player().chip_options.clone()[0];
    match my_game.play(0, c) {
        BoardState::Ongoing => {
            println!("EASY");
        },
        BoardState::Invalid => {
            println!("NOTHING");
        },

        BoardState::Draw => {
            println!("Draw");
        },
        BoardState::Win(x) => {
            println!("{}",x);
        },
    };

    let (_,col, chip) = evaluate_board(&mut my_game, EASY_AI);
    println!("Got move");
    match my_game.play(col, chip) {
        BoardState::Ongoing => {
            println!("EASY");
        },
        BoardState::Invalid => {
            println!("NOTHING");
        },

        BoardState::Draw => {
            println!("Draw");
        },
        BoardState::Win(x) => {
            println!("{}",x);
        },
    };

    println!("{:?}", my_game.get_board_layout());
}

fn main() {
    run();

    //test();

    //let mut x = PlayerTurn::Player2;
    //x = x.flip();
    //println!("{:?}", x);


    // let mut s = String::from("Hello my name is Anuj");

    // s.replace_range(1..1, "f");

    // println!("{}",s);
}
