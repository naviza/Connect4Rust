use yew::{prelude::*, html, Children, Component, Html, Properties};

pub struct GameBoard{
    input_column: String,
    sample_text: String,
}

pub enum GameBoardMsg {
    Update(String),
    Clear(String),
    SubmitTurn
}


impl Component for GameBoard {
    type Message = GameBoardMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            input_column:"".to_string(),
            sample_text:"".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameBoardMsg::Update(content) => {
                // let mut x = content.as_string().unwrap();
                self.input_column = self.input_column.clone() + &content;
            },
            GameBoardMsg::SubmitTurn => {
                if self.sample_text == "Richmond".to_string(){
                    self.sample_text = "RICHMOND".to_string();
                } else {
                    self.sample_text = "Richmond".to_string();
                }
                self.input_column = self.input_column.to_uppercase();
            },
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

        let onkeypress = ctx.link().callback(|event: InputEvent|{
            if event.input_type() == "insertText" {
                // GameBoardMsg::Update(event.data().unwrap());
                GameBoardMsg::Update(event.data().unwrap());
            } else if event.input_type() == "deleteContent" {
                GameBoardMsg::Clear(event.data().unwrap());
            }
            GameBoardMsg::Update(event.data().unwrap())

        }
        );
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