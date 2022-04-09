use yew::{prelude::*, html, Children, Component, Html, Properties};
use muicss_yew::input::Input;

pub struct GameBoard{
    input_column: String,
    sample_text: String,
    submitPlayerButtonDisabled: bool,
    width: usize,
    height: usize,
    game_state: String,
}

pub enum GameBoardMsg {
    Update(String),
    Clear(String),
    SubmitTurn(u8),
    TestClick,
    DoNothing
}


impl Component for GameBoard {
    type Message = GameBoardMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut initial_game_state = "".to_string();
        for i in 0..=6*7{
            initial_game_state += "0";
        }
        print_state(&initial_game_state, 6, 7);

        Self{
            input_column:"".to_string(),
            sample_text:"".to_string(),
            submitPlayerButtonDisabled: false,
            width: 7,
            height: 6,
            game_state: initial_game_state
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameBoardMsg::Update(content) => {
                // let mut x = content.as_string().unwrap();
                self.input_column = self.input_column.clone() + &content;
            },
            GameBoardMsg::SubmitTurn(col) => {
                log::info!("Submitted column: {}", col);
                self.submitPlayerButtonDisabled = true;
                self.sample_text = "Current player: ".to_owned() + &self.input_column.clone();
            },
            GameBoardMsg::Clear(_) => self.input_column = "".to_string(),
            GameBoardMsg::DoNothing => {},
            GameBoardMsg::TestClick => {log::info!("TestClick")},
        }
        true
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
        
        html! { 
            <>
                < input
                    type="text"
                    id="fname"
                    value={self.input_column.clone()}
                    oninput={onkeypress}
                />
                <button
                    onclick={ctx.link().callback(|_event: MouseEvent| GameBoardMsg::SubmitTurn(3))}
                    hidden={self.submitPlayerButtonDisabled}
                > {"Submit"} </button>
                <p> {self.sample_text.clone()} </p>

                {game_state_table}
                

                <h5 class={"w3-xxxlarge w3-text-red"}><b>{"Enter Player Names"}</b></h5>
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
        let holder = &gameState[start..end];
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
    html! {
        <td onclick={ctx.link().callback(move |_event: MouseEvent| GameBoardMsg::SubmitTurn(column))}>
            {gamepiece}
        </td>
    }
}