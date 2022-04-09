use yew::{prelude::*, html, Children, Component, Html, Properties};
use log::log;

pub struct Connect4Instruction;


impl Component for Connect4Instruction {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("We are here now. Logging works");
        html! { 
            <>
            <p> {"Connect Four is a two-player connection game in which the players take turns dropping colored discs from "} <br/>
                    {"the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the "} <br/>
                    {"first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
            </p>

            <p>{"To play Connect 4 follow the following steps:"}</p>
            <ul>
                <li>{"A new game describes discs of which color belongs to which player"}</li>
                <li>{"Click on the desired column on the game board to place your disc"}</li>
                <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}{""}</li>
            </ul>
            
            <p>{"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a></p>
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

