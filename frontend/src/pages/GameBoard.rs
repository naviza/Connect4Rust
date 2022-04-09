use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use connect4_lib::io::*;
use connect4_lib::play;
use yew::{html, prelude::*, Children, Component, Html, Properties};

pub struct GameBoard {
    input_column: String,
    sample_text: String,
    game_state: String,
    internal_game: Game,
    turn: PlayerTurn, // This should be kept track of in the update function, so no check is needed in maketurn()
    ai: MyAi,
}

#[derive(Clone, Copy,PartialEq)]
pub enum MyAi {
    Hard,
    Med,
    Easy,
    Human,
}

#[derive(Clone, Copy,PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

pub enum GameBoardMsg {
    Update(String),
    Clear(String),
    SubmitTurn,
}

impl PlayerTurn {
    pub fn flip(mut self) -> Self{
        match self {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        }
    }
}

impl GameBoard {
    fn find_i(state: &mut Vec<char>, col: isize, w: isize, index: isize, player: char) -> i32 {
        if (index as usize) >= state.len() {
            return -1;
        }
        let my_col: isize = (index + 1) % w;
        //last index before going to the next row
        if my_col == col {
            if state[index as usize] == '0' {
                if GameBoard::find_i(state, col, w, index + w, player) == -1 {
                    state[index as usize] = player;
                    return 0;
                } else {
                    return 0;
                }
            } else {
                //change the pervious state of it
                state[(index - w) as usize] = player;
                return 0;
            }
        } else {
            GameBoard::find_i(state, col, w, index + 1, player)
        }
    }

    fn make_ai_turn(&mut self) -> BoardState {
        let board = self.internal_game.get_board().clone();
        let chip_descrip = self.internal_game.current_player().chip_options[0];

        let mut col = 0;

        match self.ai.clone() {
            MyAi::Easy => {
                (col, _) = get_best_move(&mut self.internal_game, EASY_AI);
            }
            MyAi::Med => {
                (col, _) = get_best_move(&mut self.internal_game, MID_AI);
            }
            MyAi::Hard => {
                (col, _) = get_best_move(&mut self.internal_game, HARD_AI);
            }
            _ => {
                //should never be human
            }
        }

        // make the play
        let state = self.internal_game.play(col, chip_descrip);

        match state {
            BoardState::Ongoing => {
                let w = self.internal_game.get_board().width();
                let mut gstate: Vec<char> = self.game_state.chars().collect();
                GameBoard::find_i(&mut gstate, col, w.try_into().unwrap(), 0, '2');
                self.game_state = gstate.into_iter().collect();
                state
            }

            //if its either someone wins, draws, or invalid, and the function that called this will handle it
            _ => state,
        }
    }

    pub fn make_turn(&mut self, col: isize) -> BoardState {
        let ai = self.ai.clone();
        match ai {
            MyAi::Human => self.make_turn_helper(col),
            _ => {
                // any difficulty for the AI
                let state = self.make_turn_helper(col).clone(); // does the humans turn
                match state {
                    BoardState::Ongoing => {
                        // player 1's turn was good, the AI can make his move now
                        self.make_ai_turn()
                    }

                    _ => state, //player 1's turn was not good, let the update function handle it
                }
            }
        }
    }

    pub fn make_turn_helper(&mut self, col: isize) -> BoardState {
        let board = self.internal_game.get_board().clone();
        let chip_descrip = self.internal_game.current_player().chip_options[0];

        // make the play
        let state = self.internal_game.play(col, chip_descrip);
        match state {
            BoardState::Ongoing => {
                let w = self.internal_game.get_board().width();
                let mut gstate: Vec<char> = self.game_state.chars().collect();
                GameBoard::find_i(
                    &mut gstate,
                    col,
                    w.try_into().unwrap(),
                    0,
                    match self.turn {
                        PlayerTurn::Player1 => '1',
                        PlayerTurn::Player2 => '2',
                    },
                );
                self.game_state = gstate.into_iter().collect();
                state
            }

            //if its either someone wins, draws, or invalid, and the function that called this will handle it
            _ => state,
        }
    }
}

impl Component for GameBoard {
    type Message = GameBoardMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut g_state = "".to_string();
        let width = 7;
        let height = 6;
        for _ in 0..(width * height - 1) {
            g_state += "0";
        }
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

        let player1 = Player {
            player_type: PlayerType::Local,
            chip_options: vec![p1_chips],
            win_conditions: vec![
                vec![p1_chips],
                vec![p1_chips],
                vec![p1_chips],
                vec![p1_chips],
            ],
        };
        let player2 = Player {
            player_type: PlayerType::AI(HARD_AI),
            chip_options: vec![p2_chips],
            win_conditions: vec![
                vec![p2_chips],
                vec![p2_chips],
                vec![p2_chips],
                vec![p2_chips],
            ],
        };

        Self {
            input_column: "".to_string(),
            sample_text: "".to_string(),
            game_state: g_state,
            internal_game: Game::new(board, vec![player1, player2]),
            turn: PlayerTurn::Player1,
            ai: MyAi::Hard,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameBoardMsg::Update(content) => {
                // let mut x = content.as_string().unwrap();
                self.input_column = self.input_column.clone() + &content;
            }
            GameBoardMsg::SubmitTurn => {
                if self.sample_text == "Richmond".to_string() {
                    self.sample_text = "RICHMOND".to_string();
                } else {
                    self.sample_text = "Richmond".to_string();
                }
                self.input_column = self.input_column.to_uppercase();

                let col = 5; //change to the input column
                match self.make_turn(col) {
                    BoardState::Ongoing => {
                        //valid move by the player, we can now end the current players turn
                        if self.ai == MyAi::Human { // if its humans then flip the users turn, if there is an AI, make_turn handles it
                            //Only Humans
                            self.turn = self.turn.flip();
                        }
                    }
                    BoardState::Invalid => {
                        //invalid move by current player
                    }
                    BoardState::Draw => {
                        //the game is a draw
                    }
                    BoardState::Win(x) => {
                        //player x has won
                    }
                }
            }
            GameBoardMsg::Clear(_) => self.input_column = "".to_string(),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().callback(|event: InputEvent| {
            if event.input_type() == "insertText" {
                // GameBoardMsg::Update(event.data().unwrap());
                GameBoardMsg::Update(event.data().unwrap());
            } else if event.input_type() == "deleteContent" {
                GameBoardMsg::Clear(event.data().unwrap());
            }
            GameBoardMsg::Update(event.data().unwrap())
        });
        // let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
        //     if e.key() == "Enter" {
        //         // let input: InputEvent = e.target_unchecked_into();
        //         // let value = input.data();
        //         // input.set_value("");
        //         // Some(Msg::Add(value))
        //         // GameBoardMsg::Update("".to_string());
        //     }
        //     GameBoardMsg::Update("as".to_string())
        // });

        html! {
            <>
                < input
                    type="text"
                    id="fname"
                    value={self.input_column.clone()}
                    // oninput={ctx.link().callback(|event: InputEvent| GameBoardMsg::Update(event))}
                    oninput={onkeypress}

                    />
                    <button onclick={ctx.link().callback(|event: MouseEvent| GameBoardMsg::SubmitTurn)}>
                    //      ^^^^^^^ event listener name
                        { "Click me!" }
                    </button>
                <p> {self.sample_text.clone()} </p>
            </>
        }
    }
}
