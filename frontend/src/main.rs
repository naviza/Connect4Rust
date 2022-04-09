use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use log::log;

mod pages;

use crate::pages::Connect4Instruction::Connect4Instruction;
use crate::pages::Connect4Computer::Connect4Computer;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/instructions")]
    Instructions,
    #[at("/connect_four_vs_human")]
    C4Human,
    #[at("/connect_four_vs_computer")]
    C4Computer,
    #[not_found]
    #[at("/404")]
    NotFound,
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
        <h1>{ "Connect 4 against an actual Human" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        </div>
    }
}

#[function_component(C4Computer)]
fn c4_against_computer() -> Html {
    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));

    html! {
        <div>
        <h1>{ "Connect 4 against a Computer" }</h1> <br />
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <br /><br />
        <Connect4Computer />
        </div>
    }
}

#[function_component(Instructions)]
fn instructions() -> Html {

    let history = use_history().unwrap();
    let onclick_callback1 = Callback::from(move |_| history.push(Route::Home));



    html! {
        <div>
        <h1>{ "Instructions" }</h1>
        <button onclick={onclick_callback1}>{ "Go to Home" }</button>
        <Connect4Instruction />
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let mut history = use_history().unwrap();

    let onclick_callback1 = Callback::from(move |_| history.push(Route::Instructions));
    history = use_history().unwrap().clone();
    let onclick_callback2 = Callback::from(move |_| history.push(Route::C4Human));
    history = use_history().unwrap().clone();
    let onclick_callback3 = Callback::from(move |_| history.push(Route::C4Computer));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <div class="block-display">
                <button onclick={onclick_callback1} class={"w3-button w3-blue w3-jumbo"}>{ "Instructions for Connect 4" }</button>
                <button onclick={onclick_callback2} class={"w3-button w3-blue w3-jumbo"}>{ "Play against a Human" }</button>
                <button onclick={onclick_callback3} class={"w3-button w3-blue w3-jumbo"}>{ "Play against a Computer" }</button>
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
        Route::Instructions => html! {
            <Instructions />
        },
        Route::C4Human => html! {
            <C4Human />
        },
        Route::C4Computer => html! {
            <C4Computer />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
