use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use log::log;

mod pages;

use crate::pages::Connect4Instruction::Connect4Instruction;
use crate::pages::Connect4Computer::Connect4Computer;
use crate::pages::TootOttoInstruction::TootOttoInstruction;
use crate::pages::GameBoard::GameBoard;
use crate::pages::GameBoard::GameType;
use crate::pages::TooTGameBoard::TooTGameType;
use crate::pages::TooTGameBoard::TooTGameBoard;
use crate::pages::ScoreBoard::ScoreBoard;
use crate::pages::GameHistory::GameHistory;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/connect_four_instructions")]
    C4Instructions,
    #[at("/connect_four_vs_human")]
    C4Human,
    #[at("/connect_four_vs_computer")]
    C4Computer,
    #[at("/toot_otto_instructions")]
    TootOttoInstructions,
    #[at("/toot_otto_vs_human")]
    TootOttoHuman,
    #[at("/toot_otto_vs_computer")]
    TootOttoComputer,
    #[not_found]
    #[at("/404")]
    NotFound,
    
    #[at("/GameHistory")]
    GameHistory,
    #[at("/ScoreBoard")]
    ScoreBoard,
}


#[function_component(TootOttoInstructions)]
fn toot_otto_instructions() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <h1>{ "Toot-Otto Instructions" }</h1>
        <TootOttoInstruction />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(TootOttoHuman)]
fn toot_otto_against_human() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html!{
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <h1>{ "Toot-Otto 2 Players" }</h1>
        <TooTGameBoard game_type={TooTGameType::TooTOttO} number_of_players=2 />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go Home" }</button>
    </div>}
}

#[function_component(TootOttoComputer)]
fn toot_otto_against_computer() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html!{<div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <h1>{ "Toot-Otto 1 Player" }</h1>
        <TooTGameBoard game_type={TooTGameType::TooTOttO} number_of_players=1 />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go Home" }</button>
    </div>}
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Hey" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(C4Human)]
fn c4_against_human() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <h1>{ "Connect4 2 players" }</h1>
        <GameBoard game_type={GameType::Connect4} number_of_players=2 />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(C4Computer)]
fn c4_against_computer() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <h1>{ "Connect4 1 player" }</h1>
        <br /><br />
        <GameBoard game_type={GameType::Connect4} number_of_players=1 />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(Instructions)]
fn instructions() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
            <h1 >{ "Connect4 Instructions" }</h1>
            <Connect4Instruction />
            <br/ ><br/ >
            <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(GameHistoryMain)]
fn game_history_main() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <GameHistory />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(ScoreBoardMain)]
fn score_board_main() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div class="w3-panel w3-padding-64" style="padding-left: 80px">
        <ScoreBoard />
        <br/ ><br/ >
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let mut history = use_history().unwrap();
    
    // For Connect 4
    let onclick_callback1 = Callback::from(move |_| history.push(Route::C4Instructions));
    history = use_history().unwrap().clone();
    let onclick_callback2 = Callback::from(move |_| history.push(Route::C4Human));
    history = use_history().unwrap().clone();
    let onclick_callback3 = Callback::from(move |_| history.push(Route::C4Computer));

    // For Toot Otto
    history = use_history().unwrap().clone();
    let onclick_callback_tinstr = Callback::from(move |_| history.push(Route::TootOttoInstructions));
    history = use_history().unwrap().clone();
    let onclick_callback_thuman = Callback::from(move |_| history.push(Route::TootOttoHuman));
    history = use_history().unwrap().clone();
    let onclick_callback_tcomp = Callback::from(move |_| history.push(Route::TootOttoComputer));

    history = use_history().unwrap().clone();
    let onclick_callback_game_history = Callback::from(move |_| history.push(Route::GameHistory));
    history = use_history().unwrap().clone();
    let onclick_callback_score_board = Callback::from(move |_| history.push(Route::ScoreBoard));


    html! {
        <div class="w3-padding-32">
            <h1 class="w3-container w3-center" style="font-size:60px;">{ "Home" }</h1>
            
            // <div class="block-display">
            <div class="center">
                <button onclick={onclick_callback1} class={"w3-xxlarge w3-button w3-blue w3-block"}>{ "Instructions for Connect 4" }</button>
                <button onclick={onclick_callback2} class={"w3-xxlarge w3-button w3-blue w3-block"}>{ "Play against a Human" }</button>
                <button onclick={onclick_callback3} class={"w3-xxlarge w3-button w3-blue w3-block"}>{ "Play against a Computer" }</button>

                <button onclick={onclick_callback_tinstr} class={"w3-xxlarge w3-button w3-red w3-block"}>{ "Instructions for Toot-Otto" }</button>
                <button onclick={onclick_callback_thuman} class={"w3-xxlarge w3-button w3-red w3-block"}>{ "Play against a Human" }</button>
                <button onclick={onclick_callback_tcomp} class={"w3-xxlarge w3-button w3-red w3-block"}>{ "Play against a Computer" }</button>

                <button onclick={onclick_callback_game_history} class={"w3-xxlarge w3-button w3-green w3-block"}>{ "View Game History" }</button>
                <button onclick={onclick_callback_score_board} class={"w3-xxlarge w3-button w3-green w3-block"}>{ "Score Board" }</button>
            </div>
        </div>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::C4Instructions => html! {
            <Instructions />
        },
        Route::C4Human => html! {
            <C4Human />
        },
        Route::C4Computer => html! {
            <C4Computer />
        },
        Route::TootOttoInstructions => html! {
            <TootOttoInstructions />
        },
        Route::TootOttoHuman => html! {
            <TootOttoHuman />
        },
        Route::TootOttoComputer => html! {
            <TootOttoComputer />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::GameHistory => html! {
            <GameHistoryMain />
        },
        Route::ScoreBoard => html! {
            <ScoreBoardMain />
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
