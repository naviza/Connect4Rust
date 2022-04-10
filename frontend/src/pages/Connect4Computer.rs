use yew::{prelude::*, html, Children, Component, Html, Properties};
use crate::pages::GameBoard::GameBoard;
use crate::pages::GameBoard::GameType;

pub struct Connect4Computer;


impl Component for Connect4Computer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { 
            <>
                <GameBoard game_type={GameType::Connect4} number_of_players=1 />
            </>
        }
    }
}