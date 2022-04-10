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
        <div>
        <h1>{ "Toot-Otto Instructions" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <TootOttoInstruction />
        </div>
    }
}

#[function_component(TootOttoHuman)]
fn toot_otto_against_human() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html!{
    <>
        <h1>{ "Toot-Otto 2 Players" }</h1>
        <button onclick={onclick_callback1}>{ "Go Home" }</button>
        <TooTGameBoard game_type={TooTGameType::TooTOttO} number_of_players=2 />
    </>}
}

#[function_component(TootOttoComputer)]
fn toot_otto_against_computer() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html!{<>
        <h1>{ "Toot-Otto 1 Player" }</h1>
        <button onclick={onclick_callback1}>{ "Go Home" }</button>
        <TooTGameBoard game_type={TooTGameType::TooTOttO} number_of_players=1 />
    </>}
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
        <div>
        <h1>{ "Connect4 2 players" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <GameBoard game_type={GameType::Connect4} number_of_players=2 />
        </div>
    }
}

#[function_component(C4Computer)]
fn c4_against_computer() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
        <h1>{ "Connect4 1 player" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <br /><br />
        <GameBoard game_type={GameType::Connect4} number_of_players=1 />
        </div>
    }
}

#[function_component(Instructions)]
fn instructions() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
        <h1>{ "Connect4 Instructions" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <Connect4Instruction />
        </div>
    }
}

#[function_component(GameHistoryMain)]
fn game_history_main() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <GameHistory />
        </div>
    }
}

#[function_component(ScoreBoardMain)]
fn score_board_main() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));
    html! {
        <div>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <ScoreBoard />
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
        <div>
            <h1>{ "Home" }</h1>
            
            // <div class="block-display">
            <div class="w3-sidebar w3-bar-block" style="width:25%">
                <button onclick={onclick_callback1} class={"w3-bar-item w3-button w3-blue"}>{ "Instructions for Connect 4" }</button>
                <button onclick={onclick_callback2} class={"w3-bar-item w3-button w3-blue"}>{ "Play against a Human" }</button>
                <button onclick={onclick_callback3} class={"w3-bar-item w3-button w3-blue"}>{ "Play against a Computer" }</button>

                <button onclick={onclick_callback_tinstr} class={"w3-bar-item w3-button w3-red"}>{ "Instructions for Toot-Otto" }</button>
                <button onclick={onclick_callback_thuman} class={"w3-bar-item w3-button w3-red"}>{ "Play against a Human" }</button>
                <button onclick={onclick_callback_tcomp} class={"w3-bar-item w3-button w3-red"}>{ "Play against a Computer" }</button>

                <button onclick={onclick_callback_game_history} class={"w3-bar-item w3-button w3-green"}>{ "View Game History" }</button>
                <button onclick={onclick_callback_score_board} class={"w3-bar-item w3-button w3-green"}>{ "Score Board" }</button>
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
