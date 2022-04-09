use yew::{prelude::*, html, Children, Component, Html, Properties};
use log::log;

pub struct GameHistory;

#[derive(Clone,PartialEq, Debug)]
pub struct GameData{
    id: String,
    gameType: String,
    player1: String,
    player2: String,
    winner: String,
    whenPlayed: String,
}


impl Component for GameHistory {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut veccer = Vec::new();
        veccer.push(
            GameData{
                id: "String".to_string(),
                gameType: "String".to_string(),
                player1: "String".to_string(),
                player2: "String".to_string(),
                winner: "String".to_string(),
                whenPlayed: "String".to_string(),
            }
        );

        veccer.push(
            GameData{
                id: "String".to_string(),
                gameType: "String".to_string(),
                player1: "String".to_string(),
                player2: "String".to_string(),
                winner: "String".to_string(),
                whenPlayed: "String".to_string(),
            }
        );
        
        let history_table = make_history_table(&ctx, veccer);
        html! { 
            <>
            <h1>{"Game History"}</h1>
            {history_table}
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

fn make_history_row(ctx: &Context<GameHistory>, data: &GameData) -> Html {
    html!{
        <tr>
            <td>{data.id.clone()}</td>
            <td>{data.gameType.clone()}</td>
            <td>{data.player1.clone()}</td>
            <td>{data.player2.clone()}</td>
            <td>{data.winner.clone()}</td>
            <td>{data.whenPlayed.clone()}</td>
        </tr>
    }
}

fn make_history_table(ctx: &Context<GameHistory>, datum: Vec<GameData>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_history_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"}>
            <tr>
                <th>{"Game-ID"}</th>
                <th>{"Game Type"}</th>
                <th>{"Player1"}</th>
                <th>{"Player2"}</th>
                <th>{"Winner"}</th>
                <th>{"When Played"}</th>
            </tr>
            {game_rows}
        </table>
    }
}