use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

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

#[function_component(Instructions)]
fn instructions() -> Html {
    let history = use_history().unwrap();
    html! {
        <h1>{ "Instructions" }</h1>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();

    let onclick_callback = Callback::from(move |_| history.push(Route::Instructions));
    //let onclick_callback1 = Callback::from(move |_| history.push(Route::C4Human));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={onclick_callback}>{ "Instructions for Connect 4" }</button>
            //<button onclick={onclick_callback1}>{ "Play against a Human" }</button>
            //<button onclick={Callback::from(move |_| history.push(Route::C4Computer))}>{ "Play against a Computer" }</button>
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
            <Home />
        },
        Route::C4Computer => html! {
            <Home />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },

    }
}

fn main() {
    yew::start_app::<Main>();
}
