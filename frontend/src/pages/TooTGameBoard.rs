use connect4_lib::ai::*;
use connect4_lib::game::*;
use connect4_lib::games::*;
use ran::*;

use yew::{html, prelude::*, Children, Component, Html, Properties};

pub struct TooTGameBoard {
    player1_name_input: String,
    player2_name_input: String,
    submit_player1_button_disabled: bool,
    submit_player2_button_disabled: bool,
    game_state: String,
    internal_game: Game,
    turn: PlayerTurn, // This should be kept track of in the update function, so no check is needed in maketurn()
    ai: MyAi,
    width: usize,
    height: usize,
    game_type: TooTGameType,
    number_of_players: usize,
    game_is_done: bool,
    winner_name: String,

    current_letter: char,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MyAi {
    Hard(isize,isize),
    Med(isize,isize),
    Easy(isize,isize),
    Human,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

pub enum TooTGameBoardMsg {
    Player1NameUpdate(String),
    Player2NameUpdate(String),
    SubmitPlayer1,
    SubmitPlayer2,

    Clear(String),
    SubmitTurn(u8),
    TestClick,
    DoNothing,
    IncreaseAIDifficulty,
    DecreaseAIDifficulty,

    MakeOTurn,
    MakeTTurn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TooTGameType {
    TooTOttO,
    Connect4,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TooTGameBoardProps {
    pub game_type: TooTGameType,
    pub number_of_players: usize,
}

impl PlayerTurn {
    pub fn flip(self) -> Self {
        match self {
            PlayerTurn::Player1 => PlayerTurn::Player2,
            PlayerTurn::Player2 => PlayerTurn::Player1,
        }
    }
}

pub fn my_evaluate_board(game: &mut Game, ite : isize, depth: isize) -> (isize, isize, ChipDescrip) {
    let is_max = game.get_turn() % 2 == 0;

    fn my_test_move(mov: isize, chip: ChipDescrip, game: &mut Game, ite : isize, depth: isize) -> isize {
        game.play(mov, chip);
        let mut score = my_minmax_search(game, depth) << (14 as isize);
        if score == 0 {
            score = my_monte_carlo_search(game, ite);
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
        .map(|(mov, c)| (my_test_move(mov, c, &mut game.clone(), ite, depth), mov, c))
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

fn my_monte_carlo_search(game: &mut Game, iter: isize) -> isize {
    let mut score = 0;
    (0..iter).for_each(|_| {
        let mut moves = 0;
        let mut res = BoardState::Ongoing;
        let mut finished = false;
        while !finished {
            match res {
                BoardState::Ongoing => {
                    let m = game.get_board().get_valid_moves();
                    log::info!("STILL CHOOSING");
                    let lb = 0;
                    let up = m.len() as i64 - 1;
                    let r : usize = ran_irange(lb, up) as usize;
                    let mov = m[r];
                    let up1 = game.current_player().chip_options.len() as i64 - 1;
                    let r : usize = ran_irange(lb, up1) as usize;
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

impl TooTGameBoard {
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
            if v[i] == 120 {
                //byte for x
                //first index to have x in it, so the bottom
                index = i;
                break;
            }
        }
        if self.height + 1 == index {
            //no empty spot in this column
            //internal game should take care of this
        } else {
            let ind = index * self.width + col;

            state.replace_range(ind..ind + 1, player);
        }
        state
    }

    fn make_ai_turn(&mut self) -> BoardState {
        log::info!("MADE IT TO AI TURN");
        let (iter, depth) = match self.ai.clone() {
            MyAi::Easy(i,d) => (i,d),
            MyAi::Med(i,d) => (i,d),
            MyAi::Hard(i,d) => (i,d),
            _ => (0,0),
        };

        let (_,col, chip) = my_evaluate_board(&mut self.internal_game, iter, depth);
        // make the play
        log::info!("col = {}", col);
        let state = self.internal_game.play(col, chip);

        match state {
            BoardState::Ongoing => {
                self.game_state = self.find(col as usize, &chip.graphic.to_string()[..]);
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

        let chip_descrip = if self.current_letter == 'T' {
            ChipDescrip {
                bg_color: 60,
                fg_color: 1,
                graphic: 'T',
            }
        } else {
            ChipDescrip {
                bg_color: 60,
                fg_color: 1,
                graphic: 'O',
            }
        };

        // make the play
        let state = self.internal_game.play(col as isize, chip_descrip);
        match state {
            BoardState::Ongoing => {
                log::info!("Good Move!");
                log::info!("Finding i ...");
                self.game_state = self.find(col, &self.current_letter.to_string()[..]);
                toot_print_state(&self.game_state, self.height, self.width);
                state
            }

            //if its either someone wins, draws, or invalid, and the function that called this will handle it
            _ => state,
        }
    }
}

impl Component for TooTGameBoard {
    type Message = TooTGameBoardMsg;
    type Properties = TooTGameBoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut g_state = "".to_string();
        let width: isize = 7;
        let height: isize = 6;
        for _ in 0..(width * height) {
            g_state += "x";
        }
        let board = Board::new(width, height);

        let t_chips = ChipDescrip {
            bg_color: 60,
            fg_color: 1,
            graphic: 'T',
        };
        let o_chips = ChipDescrip {
            bg_color: 60,
            fg_color: 1,
            graphic: 'O',
        };
        let co1 = wrap_4_check(t_chips, o_chips);
        let co2 = wrap_4_check(o_chips, t_chips);
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

        log::info!(
            "Started: {:?} with {} player(s).",
            ctx.props().game_type,
            ctx.props().number_of_players
        );

        let ai = if ctx.props().number_of_players == 2 {
            MyAi::Human
        } else {
            MyAi::Med(1000,4)
        };
        Self {
            player1_name_input: "".to_string(),
            player2_name_input: "".to_string(),
            submit_player1_button_disabled: false,
            submit_player2_button_disabled: false,
            game_state: g_state,
            internal_game: Game::new(board, vec![player1, player2]),
            turn: PlayerTurn::Player1,
            ai: ai,
            width: width as usize,
            height: height as usize,
            number_of_players: ctx.props().number_of_players,
            game_type: ctx.props().game_type.clone(),
            game_is_done: false,
            winner_name: "".to_string(),
            current_letter: 'T',
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TooTGameBoardMsg::Player1NameUpdate(content) => {
                // let mut x = content.as_string().unwrap();
                self.player1_name_input = self.player1_name_input.clone() + &content;
                return true;
            }
            TooTGameBoardMsg::Player2NameUpdate(content) => {
                // let mut x = content.as_string().unwrap();
                self.player2_name_input = self.player2_name_input.clone() + &content;
                return true;
            }
            TooTGameBoardMsg::SubmitTurn(col) => {
                toot_print_state(&self.game_state, self.height, self.width);
                log::info!("Submitted column: {}, letter: {}", col, self.current_letter);

                // let col = 5; //change to the input column
                match self.make_turn(col as usize) {
                    BoardState::Ongoing => {
                        log::info!("It shouldnt make it here in 4");
                        //valid move by the player, we can now end the current players turn
                        if self.ai == MyAi::Human {
                            // if its humans then flip the users turn, if there is an AI, make_turn handles it
                            //Only Humans
                            self.turn = self.turn.flip();
                        }
                    }
                    BoardState::Invalid => {
                        //invalid move by current player
                        log::info!("The player made an invalid move.");
                    }
                    BoardState::Draw => {
                        //the game is a draw --> send the game info to the database
                    }
                    BoardState::Win(x) => {
                        //player x has won
                        //the game is a draw --> send the game info to the database
                        self.game_is_done = true;
                        self.winner_name = match x {
                            1 => self.player1_name_input.clone(),
                            2 => self.player2_name_input.clone(),
                            _ => "Error".to_string(),
                        };
                        log::info!("Player {} has won the game!", x);
                        if x == 2 && self.number_of_players == 1{
                            // computer won.
                            self.winner_name = "Computer".to_string();
                        }
                    }
                }
                // print_state(&self.game_state, self.height, self.width);
                return true;
            }
            TooTGameBoardMsg::Clear(_) => {
                return true;
            }
            TooTGameBoardMsg::DoNothing => {}
            TooTGameBoardMsg::TestClick => {
                log::info!("TestClick")
            }
            TooTGameBoardMsg::SubmitPlayer1 => {
                log::info!("AI Difficulty: {:?}", self.ai);
                log::info!("Submitted player 1: {}", self.player1_name_input.clone());
                self.submit_player1_button_disabled = true;
                // self.sample_text = "Current player: ".to_owned() + &self.player1_name_input.clone();
                self.turn = PlayerTurn::Player1;
                return true;
            }
            TooTGameBoardMsg::SubmitPlayer2 => {
                log::info!("AI Difficulty: {:?}", self.ai);
                log::info!("Submitted player 2: {}", self.player2_name_input.clone());
                self.submit_player2_button_disabled = true;
                // self.sample_text = "Current player: ".to_owned() + &self.player1_name_input.clone();
                // self.turn = PlayerTurn::Player1;
                return true;
            }
            TooTGameBoardMsg::IncreaseAIDifficulty => {
                // log::info!("Increasing AI difficulty");
                match self.ai.clone() {
                    MyAi::Easy(_,_) => self.ai = MyAi::Med(1000,4),
                    MyAi::Hard(_,_) => {},
                    MyAi::Med(_,_) => self.ai = MyAi::Hard(4000,6),
                    MyAi::Human => {
                        log::info!("Error. You shouldn't be changing AI difficulty with 2 players.")
                    }
                }
                return true;
            }
            TooTGameBoardMsg::DecreaseAIDifficulty => {
                // log::info!("Decreasing AI difficulty");
                match self.ai.clone() {
                    MyAi::Easy(_,_) => {},
                    MyAi::Hard(_,_) => self.ai = MyAi::Med(1000,4),
                    MyAi::Med(_,_) => self.ai = MyAi::Easy(5,2),
                    MyAi::Human => {
                        log::info!("Error. You shouldn't be changing AI difficulty with 2 players.")
                    }
                }
                return true;
            }
            TooTGameBoardMsg::MakeOTurn => {
                self.current_letter = 'O';
                return true;
            }
            TooTGameBoardMsg::MakeTTurn => {
                self.current_letter = 'T';
                return true;
            }
        }
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().callback(|event: InputEvent| match event.data() {
            Some(text) => TooTGameBoardMsg::Player1NameUpdate(text),
            None => TooTGameBoardMsg::DoNothing,
        });

        let player1_input = html! {
            <>
                <input
                    type="text"
                    id="fname"
                    placeholder="Player 1"
                    disabled={self.submit_player1_button_disabled}
                    value={self.player1_name_input.clone()}
                    oninput={onkeypress}
                />
                <button
                    onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::SubmitPlayer1)}
                    hidden={self.submit_player1_button_disabled}
                > {"Submit"} </button>
            </>
        };

        let one_player_only = self.number_of_players == 1;
        let mut secondary_input = html! {<></>};
        if one_player_only {
            // if there's only 1 player, 2nd input is ai difficulty
            let ai_string = match self.ai {
                MyAi::Easy(_,_) => "Easy",
                MyAi::Hard(_,_) => "Hard",
                MyAi::Med(_,_) => "Medium",
                MyAi::Human => "Error",
            };
            secondary_input = html! {
                <>
                    <button class="w3-button w3-circle w3-teal" disabled={self.submit_player1_button_disabled}
                        onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::DecreaseAIDifficulty)}>{"-"}</button>
                    {ai_string}
                    <button class="w3-button w3-circle w3-teal" disabled={self.submit_player1_button_disabled}
                        onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::IncreaseAIDifficulty)}>{"+"}</button>
                </>
            };
        } else {
            // if 2 players, 2nd input is 2nd player
            let onkeypress2 = ctx.link().callback(|event: InputEvent| match event.data() {
                Some(text) => TooTGameBoardMsg::Player2NameUpdate(text),
                None => TooTGameBoardMsg::DoNothing,
            });
            secondary_input = html! {
                <>
                    <input
                        type="text"
                        id="2ndfname"
                        placeholder="Player 2"
                        disabled={self.submit_player2_button_disabled}
                        value={self.player2_name_input.clone()}
                        oninput={onkeypress2}
                    />
                    <button
                        onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::SubmitPlayer2)}
                        hidden={self.submit_player2_button_disabled}
                    > {"Submit"} </button>
                </>
            };
        }

        let game_state_table = toot_render_table(&self.game_state, self.width, self.height, &ctx);
        let mut game_table = html! {<></>};
        if one_player_only {
            if self.submit_player1_button_disabled {
                // if true, game has started
                // log::info!("Game started");
                game_table = toot_render_table(&self.game_state, self.width, self.height, &ctx);
            }
        } else {
            if self.submit_player1_button_disabled && self.submit_player2_button_disabled {
                // if true, game has started
                // log::info!("Game started");
                game_table = toot_render_table(&self.game_state, self.width, self.height, &ctx);
            }
        }

        let mut turn_letter = html! {
            <>
                <button onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::MakeTTurn)}>{"T"}</button>
                <button onclick={ctx.link().callback(|_event: MouseEvent| TooTGameBoardMsg::MakeOTurn)}>{"O"}</button>
                <p>{format!("Current Selection: {}", self.current_letter)}</p>
            </>
        };

        if self.game_is_done {
            let restart_string = match self.number_of_players{
                1 => "/toot_otto_vs_computer".to_string(),
                2 => "/toot_otto_vs_human".to_string(),
                _ => "/".to_string()
            };
            return html! {
                <>
                    <h5 class={"w3-text-green"}>{format!("The winner is: {}", self.winner_name.clone())}</h5>
                    <a href={restart_string}>{"Click here to restart the game"}</a>
                </>
            };
        };

        html! {
            <>
                <h5 class={"w3-text-blue"}><b>{"Enter Player Name"}</b></h5>
                {player1_input} <br />
                {secondary_input}
                <p> </p>
                {turn_letter}
                {game_table}
            </>
        }
    }
}

fn toot_print_state(game_state: &String, height: usize, width: usize) -> () {
    let mut state_holder = "\n".to_string();
    for i in 0..height {
        let start = width * i;
        let end = width * (i + 1) - 1;
        state_holder += &game_state[start..=end];
        state_holder += "\n";
    }
    log::info!("{}", state_holder);
}

fn toot_render_row(
    game_row: &str,
    ctx: &Context<TooTGameBoard>,
) -> Html {
    let mut vec_holder = Vec::new();
    let my_vec: Vec<char> = game_row.chars().collect();
    for i in 0..my_vec.len() {
        let column_choice = i as u8;
        vec_holder.push(toot_render_cell(my_vec[i], ctx, column_choice));
    }
    html! {
        <tr>
            {vec_holder}
        </tr>
    }
}

fn toot_render_table(
    game_state: &String,
    width: usize,
    height: usize,
    ctx: &Context<TooTGameBoard>,
) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..height {
        let start = width * i;
        let end = width * (i + 1) - 1;
        // log::info!("start {}", start);
        // log::info!("end {}", end);
        let holder = &game_state[start..=end];
        game_rows.push(toot_render_row(&holder, &ctx));
    }
    html! {
        <table border={"1"}>
                {game_rows}
        </table>
    }
}

fn toot_render_cell(gamepiece: char, ctx: &Context<TooTGameBoard>, column: u8) -> Html {
    // let turn_choice = *column;
    if gamepiece == '1' {
        return html! {
            <td
                onclick={ctx.link().callback(move |_event: MouseEvent| TooTGameBoardMsg::SubmitTurn(column))}
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
        _ => "w3-container w3-white",
    };
    html! {
        <td
            onclick={ctx.link().callback(move |_event: MouseEvent| TooTGameBoardMsg::SubmitTurn(column))}
            class={cell_style}
            >
            <h2 >{gamepiece}</h2>
        </td>
    }
}
