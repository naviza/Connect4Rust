use connect4_lib::game::Game;
use yew::{prelude::*, html, Children, Component, Html, Properties};
use log::log;
use std::num::ParseIntError;
use std::io::Read;
// use reqwest::*;
// use std::io::Read;

pub struct GameHistory;

#[derive(Clone,PartialEq, Debug)]
pub struct GameData {
    id: String,
    player1: String,
    player2: String,
    gametype: String,
    winner: String,
    whenplayed: String
}

impl GameData {
    pub fn parse(data: String) -> Vec<GameData> {
        let mut games = Vec::new();
        for game in data.split("|") {
            let split: Vec<&str> = game.split(",").collect();
            let result = GameData {
                id: split[0].to_string(),
                player1: split[1].to_string(),
                player2: split[2].to_string(),
                gametype: split[3].to_string(),
                winner: split[4].to_string(),
                whenplayed: split[5].to_string()
            };
            games.push(result);
        }
        games
    }
}


impl Component for GameHistory {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        
        let veccer = make_game_history_data();

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
            <td style={"padding:8px"}>{data.id.clone()}</td>
            <td style={"padding:8px"}>{data.gametype.clone()}</td>
            <td style={"padding:8px"}>{data.player1.clone()}</td>
            <td style={"padding:8px"}>{data.player2.clone()}</td>
            <td style={"padding:8px"}>{data.winner.clone()}</td>
            <td style={"padding:8px"}>{data.whenplayed.clone()}</td>
        </tr>
    }
}

fn make_history_table(ctx: &Context<GameHistory>, datum: Vec<GameData>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_history_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"} class={"w3-table w3-striped w3-bordered"}>
            <tr>
                <th style={"padding:16px"}>{"Game-ID"}</th>
                <th style={"padding:16px"}>{"Game Type"}</th>
                <th style={"padding:16px"}>{"Player1"}</th>
                <th style={"padding:16px"}>{"Player2"}</th>
                <th style={"padding:16px"}>{"Winner"}</th>
                <th style={"padding:16px"}>{"When Played"}</th>
            </tr>
            {game_rows}
        </table>
    }
}

// fn get_data() -> Result<String, Error>{
    
//     // let body = reqwest::blocking::get("https://www.rust-lang.org")?
//     //     .text()?;
//     let number = match reqwest::blocking::get("https://www.rust-lang.org"){
//         Ok(res) =>{
//             match res.text() {
//                 Ok(body) => body,
//                 Err(e) => return Err(e),
//             }
//         },
//         Err(e) => return Err(e),
//     };
//     println!("{}", number);
//     Ok("".to_string())
// }

// fn main_get_history() -> Result<(), Box<dyn std::error::Error>> {
//     let mut res = reqwest::get("https://httpbin.org/headers")?;

//     // copy the response body directly to stdout
//     std::io::copy(&mut res, &mut std::io::stdout())?;

//     Ok(())
// }

fn make_game_history_data() -> Vec<GameData> {
    let mut veccer = Vec::new();

        // let result = reqwest::get("https://api.spotify.com/v1/search");
        // log::info!("{:?}", result);

        veccer.push(
            GameData {
                id: "1".to_string(),
                player1: "Taranjot".to_string(),
                player2: "Ralph".to_string(),
                gametype: "Connect4".to_string(),
                winner: "Ralph".to_string(),
                whenplayed: "2022-04-09 20:32:40".to_string(),
            }
        );
        
        veccer.push(
            GameData {
                id: "2".to_string(),
                player1: "Anuj".to_string(),
                player2: "Ralph".to_string(),
                gametype: "Toot-Otto".to_string(),
                winner: "Anuj".to_string(),
                whenplayed: "2022-04-09 20:18:24".to_string(),
            }
        );

        veccer.push(
            GameData {
                id: "3".to_string(),
                player1: "Anuj".to_string(),
                player2: "Computer".to_string(),
                gametype: "Toot-Otto".to_string(),
                winner: "Computer".to_string(),
                whenplayed: "2022-04-09 20:32:40".to_string(),
            }
        );
        
        veccer.push(
            GameData {
                id: "4".to_string(),
                player1: "Anuj".to_string(),
                player2: "Richmond".to_string(),
                gametype: "Toot-Otto".to_string(),
                winner: "Anuj".to_string(),
                whenplayed: "2022-04-09 20:18:24".to_string(),
            }
        );
        veccer.push(
            GameData {
                id: "5".to_string(),
                player1: "Richmond".to_string(),
                player2: "Computer".to_string(),
                gametype: "Connect4".to_string(),
                winner: "Computer".to_string(),
                whenplayed: "2022-04-09 21:22:37".to_string(),
            }
        );
        veccer.push(
            GameData {
                id: "6".to_string(),
                player1: "Taranjot".to_string(),
                player2: "Computer".to_string(),
                gametype: "Toot-Otto".to_string(),
                winner: "Taranjot".to_string(),
                whenplayed: "2022-04-09 21:22:37".to_string(),
            }
        );

        veccer
}