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

<<<<<<< Updated upstream
#[derive(Clone, Copy,PartialEq)]
=======
#[derive(Clone, Copy, PartialEq, Debug)]
>>>>>>> Stashed changes
pub enum MyAi {
    Hard,
    Med,
    Easy,
    Human,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

pub enum GameBoardMsg {
    Update(String),
    Clear(String),
<<<<<<< Updated upstream
    SubmitTurn,
=======
    SubmitTurn(u8),
    TestClick,
    DoNothing,
    IncreaseAIDifficulty,
    DecreaseAIDifficulty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameType {
    TooTOttO,
    Connect4,
}

#[derive(Clone, PartialEq, Properties)]
pub struct GameBoardProps {
    pub game_type: GameType,
    pub number_of_players: usize,
>>>>>>> Stashed changes
}

impl PlayerTurn {
    pub fn flip(mut self) -> Self {
        match self {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        }
    }
}

impl GameBoard {
    fn find(&mut self, col: usize, player: &str) -> String {
        log::info!(" FIND FUNCTION");
        let mut state = self.game_state.clone();
        let temp = state.as_bytes();
        let mut v: Vec<u8> = vec![];
        for i in 0..self.height {
            let ind = i * self.width + col;
            log::info!(" temp[ind] = {}", temp[ind]);
            v.push(temp[ind]);
        }
        let mut index = self.height + 1;
        for i in (0..v.len()).rev() {
            if v[i] == 111 {
                //byte for o
                //first index to have o in it, so the bottom
                index = i;
                break;
            }
        }
        if self.height + 1 == index {
            //no empty spot in this column
            //internal game should take care of this
        } else {
            let ind = index * self.width + col;

            state.replace_range(ind..ind+1, player);
        }
        state
    }

    fn make_ai_turn(&mut self) -> BoardState {
        log::info!("MADE IT TO AI TURN");
        let mut chip = ChipDescrip {
            bg_color: 60,
            fg_color: 5,
            graphic: '◼',
        };
        let mut col = 0;
        log::info!("MADE IT past");
        match self.ai.clone() {
            MyAi::Easy => {
                log::info!("HERE I AM - easy");
                (_,col, chip) = evaluate_board(&mut self.internal_game, EASY_AI);
                log::info!("HERE I AM - easy");
            }
            MyAi::Med => {
                log::info!("HERE I AM - med");
                (_,col, chip) = evaluate_board(&mut self.internal_game, MID_AI);
            }
            MyAi::Hard => {
                log::info!("HERE I AM - hard");
                (_,col, chip) = evaluate_board(&mut self.internal_game, HARD_AI);
                
            }
            _ => {
                //should never be human
            }
        }

        // make the play
        log::info!("col = {}", col);
        let state = self.internal_game.play(col, chip);


        match state {
            BoardState::Ongoing => {

                self.game_state = self.find(col as usize, "2");
                log::info!("game state from ai turn = {}", self.game_state);
                state
            }

            //if its either someone wins, draws, or invalid, and the function that called this will handle it
            _ => state,
        }
    }

    pub fn make_turn(&mut self, col: usize) -> BoardState {
        let ai = self.ai.clone();
        log::info!("Make turn");
        match ai {
            MyAi::Human => self.make_turn_helper(col),
            _ => {
                log::info!("AI FOUND");
                // any difficulty for the AI
                let state = self.make_turn_helper(col).clone(); // does the humans turn
                match state {
                    BoardState::Ongoing => {
                        // player 1's turn was good, the AI can make his move now
                        log::info!("The user made a good move!");
                        self.make_ai_turn()
                    }

                    _ => state, //player 1's turn was not good, let the update function handle it
                }
            }
        }
    }

    fn make_turn_helper(&mut self, col: usize) -> BoardState {
        log::info!("Make turn helper");
        let chip_descrip = self.internal_game.current_player().chip_options[0];

        // make the play
        let state = self.internal_game.play(col as isize, chip_descrip);
        match state {
            BoardState::Ongoing => {
                log::info!("Good Move!");
                log::info!("Finding i ...");
                self.game_state = self.find(
                    col,
                    match self.turn {
                        PlayerTurn::Player1 => "1",
                        PlayerTurn::Player2 => "2",
                    },
                );
                print_state(&self.game_state,self.height ,self.width);
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
<<<<<<< Updated upstream
        let width = 7;
        let height = 6;
        for _ in 0..(width * height - 1) {
            g_state += "0";
=======
        let width: usize = 7;
        let height: usize = 6;
        for _ in 0..(width * height) {
            g_state += "o";
>>>>>>> Stashed changes
        }
        let board = Board::new(width as isize, height as isize);
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
        let co1 = four_in_a_row(p1_chips);
        let co2 = four_in_a_row(p1_chips);
        let player1 = Player {
            player_type: PlayerType::Local,
            chip_options: co1.clone(),
            win_conditions: vec![co1.clone(), co1.clone(), co1.clone(), co1.clone()],
        };
        let player2 = Player {
            player_type: PlayerType::AI(HARD_AI),
            chip_options: co2.clone(),
            win_conditions: vec![co2.clone(), co2.clone(), co2.clone(), co2.clone()],
        };

<<<<<<< Updated upstream
=======
        log::info!(
            "Started: {:?} with {} player(s).",
            ctx.props().game_type,
            ctx.props().number_of_players
        );

        let ai = if ctx.props().number_of_players == 2{ MyAi::Human} else {MyAi::Med};
>>>>>>> Stashed changes
        Self {
            input_column: "".to_string(),
            sample_text: "".to_string(),
            game_state: g_state,
            internal_game: Game::new(board, vec![player1, player2]),
            turn: PlayerTurn::Player1,
<<<<<<< Updated upstream
            ai: MyAi::Hard,
=======
            ai: ai,
            width: width,
            height: height,
            number_of_players: ctx.props().number_of_players,
            game_type: ctx.props().game_type.clone(),
>>>>>>> Stashed changes
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
<<<<<<< Updated upstream
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
=======
            GameBoardMsg::Player1NameUpdate(content) => {
                // let mut x = content.as_string().unwrap();
                self.player1_name_input = self.player1_name_input.clone() + &content;
                return true;
            }
            GameBoardMsg::Player2NameUpdate(content) => {
                // let mut x = content.as_string().unwrap();
                self.player2_name_input = self.player2_name_input.clone() + &content;
                return true;
            }
            GameBoardMsg::SubmitTurn(col) => {
                log::info!("Submitted column: {}", col);

                print_state(&self.game_state, self.height, self.width);
                // let col = 5; //change to the input column
                match self.make_turn(col as usize) {
>>>>>>> Stashed changes
                    BoardState::Ongoing => {
                        log::info!("made is into ongoing");
                        //valid move by the player, we can now end the current players turn
                        if self.ai == MyAi::Human {
                            // if its humans then flip the users turn, if there is an AI, make_turn handles it
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
<<<<<<< Updated upstream
            }
            GameBoardMsg::Clear(_) => self.input_column = "".to_string(),
=======
                // print_state(&self.game_state, self.height, self.width);
                return true;
            }
            GameBoardMsg::Clear(_) => {
                return true;
            }
            GameBoardMsg::DoNothing => {}
            GameBoardMsg::TestClick => {
                log::info!("TestClick")
            }
            GameBoardMsg::SubmitPlayer1 => {
                log::info!("AI Difficulty: {:?}", self.ai);
                log::info!("Submitted player 1: {}", self.player1_name_input.clone());
                self.submitPlayer1ButtonDisabled = true;
                // self.sample_text = "Current player: ".to_owned() + &self.player1_name_input.clone();
                self.turn = PlayerTurn::Player1;
                return true;
            }
            GameBoardMsg::SubmitPlayer2 => {
                log::info!("AI Difficulty: {:?}", self.ai);
                log::info!("Submitted player 2: {}", self.player2_name_input.clone());
                self.submitPlayer2ButtonDisabled = true;
                // self.sample_text = "Current player: ".to_owned() + &self.player1_name_input.clone();
                // self.turn = PlayerTurn::Player1;
                return true;
            }
            GameBoardMsg::IncreaseAIDifficulty => {
                // log::info!("Increasing AI difficulty");
                match self.ai.clone() {
                    MyAi::Easy => self.ai = MyAi::Med,
                    MyAi::Hard => self.ai = MyAi::Hard,
                    MyAi::Med => self.ai = MyAi::Hard,
                    MyAi::Human => {
                        log::info!("Error. You shouldn't be changing AI difficulty with 2 players.")
                    }
                }
                return true;
            }
            GameBoardMsg::DecreaseAIDifficulty => {
                // log::info!("Decreasing AI difficulty");
                match self.ai.clone() {
                    MyAi::Easy => self.ai = MyAi::Easy,
                    MyAi::Hard => self.ai = MyAi::Med,
                    MyAi::Med => self.ai = MyAi::Easy,
                    MyAi::Human => {
                        log::info!("Error. You shouldn't be changing AI difficulty with 2 players.")
                    }
                }
                return true;
            }
>>>>>>> Stashed changes
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
<<<<<<< Updated upstream
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
=======
        let onkeypress = ctx.link().callback(|event: InputEvent| match event.data() {
            Some(text) => GameBoardMsg::Player1NameUpdate(text),
            None => GameBoardMsg::DoNothing,
        });

        let player1_input = html! {
>>>>>>> Stashed changes
            <>
                < input
                    type="text"
                    id="fname"
                    value={self.input_column.clone()}
                    // oninput={ctx.link().callback(|event: InputEvent| GameBoardMsg::Update(event))}
                    oninput={onkeypress}

<<<<<<< Updated upstream
                    />
                    <button onclick={ctx.link().callback(|event: MouseEvent| GameBoardMsg::SubmitTurn)}>
                    //      ^^^^^^^ event listener name
                        { "Click me!" }
                    </button>
=======
        let one_player_only = self.number_of_players == 1;
        let mut secondary_input = html! {<></>};
        if one_player_only {
            // if there's only 1 player, 2nd input is ai difficulty
            let ai_string = match self.ai {
                MyAi::Easy => "Easy",
                MyAi::Hard => "Hard",
                MyAi::Med => "Medium",
                MyAi::Human => "Error",
            };
            secondary_input = html! {
                <>
                    <button class="w3-button w3-circle w3-teal" disabled={self.submitPlayer1ButtonDisabled}
                        onclick={ctx.link().callback(|_event: MouseEvent| GameBoardMsg::DecreaseAIDifficulty)}>{"-"}</button>
                    {ai_string}
                    <button class="w3-button w3-circle w3-teal" disabled={self.submitPlayer1ButtonDisabled}
                        onclick={ctx.link().callback(|_event: MouseEvent| GameBoardMsg::IncreaseAIDifficulty)}>{"+"}</button>
                </>
            };
        } else {
            // if 2 players, 2nd input is 2nd player
            let onkeypress2 = ctx.link().callback(|event: InputEvent| match event.data() {
                Some(text) => GameBoardMsg::Player2NameUpdate(text),
                None => GameBoardMsg::DoNothing,
            });
            secondary_input = html! {
                <>
                    <input
                        type="text"
                        id="2ndfname"
                        placeholder="Player 2"
                        disabled={self.submitPlayer2ButtonDisabled}
                        value={self.player2_name_input.clone()}
                        oninput={onkeypress2}
                    />
                    <button
                        onclick={ctx.link().callback(|_event: MouseEvent| GameBoardMsg::SubmitPlayer2)}
                        hidden={self.submitPlayer2ButtonDisabled}
                    > {"Submit"} </button>
                </>
            };
        }

        let game_state_table = render_table(&self.game_state, self.width, self.height, &ctx);
        let mut game_table = html! {<></>};
        if one_player_only {
            if self.submitPlayer1ButtonDisabled {
                // if true, game has started
                // log::info!("Game started");
                game_table = render_table(&self.game_state, self.width, self.height, &ctx);
            }
        } else {
            if self.submitPlayer1ButtonDisabled && self.submitPlayer2ButtonDisabled {
                // if true, game has started
                // log::info!("Game started");
                game_table = render_table(&self.game_state, self.width, self.height, &ctx);
            }
        }

        html! {
            <>
                <h5 class={"w3-text-red"}><b>{"Enter Player Name"}</b></h5>
                {player1_input} <br />
                {secondary_input}
>>>>>>> Stashed changes
                <p> {self.sample_text.clone()} </p>
            </>
        }
    }
}
<<<<<<< Updated upstream
=======

fn print_state(gameState: &String, height: usize, width: usize) -> () {
    let mut state_holder = "\n".to_string();
    for i in 0..height {
        let start = width * i;
        let end = width * (i + 1) - 1;
        let holder = &gameState[start..end];
        state_holder += &gameState[start..=end];
        state_holder += "\n";
    }
    log::info!("{}", state_holder);
}

fn render_row(gameRow: &str, width: usize, height: usize, ctx: &Context<GameBoard>) -> Html {
    let mut vec_holder = Vec::new();
    let my_vec: Vec<char> = gameRow.chars().collect();
    for i in 0..my_vec.len() {
        let column_choice = i as u8;
        vec_holder.push(render_cell(my_vec[i], ctx, column_choice));
    }
    html! {
        <tr>
            {vec_holder}
        </tr>
    }
}

fn render_table(gameState: &String, width: usize, height: usize, ctx: &Context<GameBoard>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..height {
        let start = width * i;
        let end = width * (i + 1) - 1;
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

fn render_cell(gamepiece: char, ctx: &Context<GameBoard>, column: u8) -> Html {
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
    }
    let cell_style = match gamepiece {
        '0' => "w3-container w3-white",
        '1' => "w3-container w3-red",
        '2' => "w3-container w3-yellow",
        't' => "w3-container w3-red",
        'o' => "w3-container w3-white",
        _ => "w3-container w3-black",
    };
    html! {
        <td
            onclick={ctx.link().callback(move |_event: MouseEvent| GameBoardMsg::SubmitTurn(column))}
            class={cell_style}
            >
            <h2 >{gamepiece}</h2>
        </td>
    }
}
>>>>>>> Stashed changes
