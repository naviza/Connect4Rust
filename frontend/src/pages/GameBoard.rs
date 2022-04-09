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
    submitPlayerButtonDisabled: bool,
    width: usize,
    height: usize,
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
    SubmitTurn(u8),
    SubmitPlayer,
    TestClick,
    DoNothing
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameType {
    TooTOttO,
    Connect4
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameBoardProps {
    pub game_type: GameType,
    pub number_of_players: usize,
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
    type Properties = GameBoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut g_state = "".to_string();
        let width = 7;
        let height = 6;
        for _ in 0..(width * height) {
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

        log::info!("Started: {:?} with {} player(s).", ctx.props().game_type, ctx.props().number_of_players);


        Self {
            input_column: "".to_string(),
            sample_text: "".to_string(),
            game_state: g_state,
            internal_game: Game::new(board, vec![player1, player2]),
            turn: PlayerTurn::Player1,
            ai: MyAi::Hard,
            submitPlayerButtonDisabled: false,
            width: 7,
            height: 6,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameBoardMsg::Update(content) => {
                // let mut x = content.as_string().unwrap();
                self.input_column = self.input_column.clone() + &content;
                return true;
            }
            GameBoardMsg::SubmitTurn(col) => {
                
                log::info!("Submitted column: {}", col);

                // let col = 5; //change to the input column
                match self.make_turn(col as isize) {
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
                // print_state(&self.game_state, self.height, self.width);
                return true;
            },
            GameBoardMsg::Clear(_) => {return true;}
            GameBoardMsg::DoNothing => {},
            GameBoardMsg::TestClick => {log::info!("TestClick")},
            GameBoardMsg::SubmitPlayer => {
                self.submitPlayerButtonDisabled = true;
                self.sample_text = "Current player: ".to_owned() + &self.input_column.clone();
                self.turn = PlayerTurn::Player1;
                return true;
            },
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {

        let onkeypress = ctx.link().callback(|event: InputEvent|{
            match event.data(){
                Some(text) => GameBoardMsg::Update(text),
                None => GameBoardMsg::DoNothing,
            }
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
        let onclickTestClick=ctx.link().callback(|_event: MouseEvent| GameBoardMsg::TestClick);

        let game_state_table = render_table(&self.game_state, self.width, self.height, &ctx);
        let mut game_table = html!{<></>};
        if self.submitPlayerButtonDisabled { // if true, game has started
            // log::info!("Game started");
            game_table = render_table(&self.game_state, self.width, self.height, &ctx);
        }
        
        html! { 
            <>
            
                <h5 class={"w3-xxxlarge w3-text-red"}><b>{"Enter Player Name"}</b></h5>
                < input
                    type="text"
                    id="fname"
                    value={self.input_column.clone()}
                    oninput={onkeypress}
                />
                <button
                    onclick={ctx.link().callback(|_event: MouseEvent| GameBoardMsg::SubmitPlayer)}
                    hidden={self.submitPlayerButtonDisabled}
                > {"Submit"} </button>
                <p> {self.sample_text.clone()} </p>
                {game_table}
            </>
        }
    }
}

fn print_state(gameState: &String, height: usize, width: usize) -> () {
    let mut state_holder = "\n".to_string();
    for i in 0..height{
        let start = width*i;
        let end = width*(i+1) - 1;
        let holder = &gameState[start..end];
        state_holder += &gameState[start..end];
        state_holder += "\n";
    }
    log::info!("{}", state_holder);
}

fn render_row(gameRow: &str, width: usize, height:usize, ctx:   &Context<GameBoard>) -> Html {
    let mut vec_holder = Vec::new();
    let my_vec: Vec<char> = gameRow.chars().collect();
    for i in 0..my_vec.len(){
        let column_choice = i as u8;
        vec_holder.push(render_cell(my_vec[i], ctx, column_choice));
    }
    html! {
        <tr>
            {vec_holder}
        </tr>
    }
}

fn render_table(gameState: &String, width: usize, height:usize, ctx:   &Context<GameBoard>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..height{
        let start = width*i;
        let end = width*(i+1) - 1;
        // log::info!("start {}", start);
        // log::info!("end {}", end);
        let holder = &gameState[start..=end];
        game_rows.push(render_row(&holder, width, height, &ctx));
    }
    html! {
        <table border={"1"}>
                {game_rows}
        </table>
    }
}

fn render_cell(gamepiece: char, ctx:   &Context<GameBoard>, column: u8) -> Html {
    // let turn_choice = *column;
    if gamepiece == '1' {
        return html! {
            <td
                onclick={ctx.link().callback(move |_event: MouseEvent| GameBoardMsg::SubmitTurn(column))}
                class={"w3-container w3-red"}
                >
                <h2 >{gamepiece}</h2>
            </td>
        };
    };
    html! {
        <td
            onclick={ctx.link().callback(move |_event: MouseEvent| GameBoardMsg::SubmitTurn(column))}
            class={"w3-container w3-green"}
            >
            <h2 >{gamepiece}</h2>
        </td>
    }
}
