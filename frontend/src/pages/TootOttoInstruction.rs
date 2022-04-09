use yew::{prelude::*, html, Children, Component, Html, Properties};
use log::log;

pub struct TootOttoInstruction;


impl Component for TootOttoInstruction {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("We are here now. Logging works");
        html! { 
            <>
            <p> {"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers."} <br/>
                    {"One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice."} <br/>
                    {"The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
            </p>

            <p>{"To play TOOT-OTTO follow the following steps:"}</p>
            <ul>
                <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                <li>{"Select the disc type T or O that you want to place"}</li>
                <li>{"Click on the desired column on the game board to place your disc"}</li>
                <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
            </ul>
            
            <p>{"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a></p>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

