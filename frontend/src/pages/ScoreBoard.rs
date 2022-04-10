use yew::{prelude::*, html, Children, Component, Html, Properties};
use log::log;

pub struct ScoreBoard;

impl Component for ScoreBoard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut comp_history = Vec::new();
        comp_history.push({
            ComputerWinGameData{
                id: "2".to_string(),
                humanplayer: "Anuj".to_string(),
                gametype: "Toot-Otto".to_string(),
                winner: "Computer".to_string(),
                whenplayed: "2022-04-09 20:32:40".to_string(),
            }
        });
        comp_history.push({
            ComputerWinGameData{
                id: "1".to_string(),
                humanplayer: "Richmond".to_string(),
                gametype: "Connect4".to_string(),
                winner: "Computer".to_string(),
                whenplayed: "2022-04-09 21:22:37".to_string(),
            }
        });
        let computer_history_table = make_won_by_computer_table(&ctx, comp_history);

        
        let mut player_wins = Vec::new();
        player_wins.push(
            PlayerWinsHistory {
                id: "1".to_string(),
                winner: "Computer".to_string(),
                wins_amount: "2".to_string(),
            }
        );
        player_wins.push(
            PlayerWinsHistory {
                id: "2".to_string(),
                winner: "Anuj".to_string(),
                wins_amount: "2".to_string(),
            }
        );
        player_wins.push(
            PlayerWinsHistory {
                id: "3".to_string(),
                winner: "Ralph".to_string(),
                wins_amount: "1".to_string(),
            }
        );
        player_wins.push(
            PlayerWinsHistory {
                id: "4".to_string(),
                winner: "Taranjot".to_string(),
                wins_amount: "1".to_string(),
            }
        );
        player_wins.push(
            PlayerWinsHistory {
                id: "5".to_string(),
                winner: "Richmond".to_string(),
                wins_amount: "0".to_string(),
            }
        );
        let player_wins_table = make_won_by_player_table(&ctx, player_wins);


        html! { 
            <>
            
                <h1>{"SCORE BOARD"}</h1>

                <h1>{"Games Won by Computer"}</h1>
                {make_games_won_by_computer_summary(&ctx, 3, 1, 2)}

                <h1>{"Details of Games Won by the Computer"}</h1>
                {computer_history_table}
                
                <h1>{"Details of Games Won by All Players"}</h1>
                {player_wins_table}
                
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

fn make_games_won_by_computer_summary(ctx: &Context<ScoreBoard>,
        total_games: u32, games_aginst:u32, games_won: u32) -> Html {
    html!{
        <table border={"1"} class={"w3-table w3-striped w3-bordered"}>
            <tr>
                <th style={"padding:16px"}>{"Total Games Played"}</th>
                <th style={"padding:16px"}>{"Games Against Computer"}</th>
                <th style={"padding:16px"}>{"Games Computer Won"}</th>
            </tr>
            <tr>
                <td style={"padding:8px"}>{format!("{}", total_games)}</td>
                <td style={"padding:8px"}>{format!("{}", games_aginst)}</td>
                <td style={"padding:8px"}>{format!("{}", games_won)}</td>
            </tr>
        </table>
    }
}


pub struct ComputerWinGameData {
    id: String,
    humanplayer: String,
    gametype: String,
    winner: String,
    whenplayed: String
}

fn make_won_by_computer_row(ctx: &Context<ScoreBoard>, data: &ComputerWinGameData) -> Html {
    html!{
        <tr>
            <td style={"padding:8px"}>{data.id.clone()}</td>
            <td style={"padding:8px"}>{data.gametype.clone()}</td>
            <td style={"padding:8px"}>{data.winner.clone()}</td>
            <td style={"padding:8px"}>{data.humanplayer.clone()}</td>
            <td style={"padding:8px"}>{data.whenplayed.clone()}</td>
        </tr>
    }
}

fn make_won_by_computer_table(ctx: &Context<ScoreBoard>, datum: Vec<ComputerWinGameData>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_won_by_computer_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"} class={"w3-table w3-striped w3-bordered"}>
            <tr>
                <th style={"padding:16px"}>{"SI. No."}</th>
                <th style={"padding:16px"}>{"Game Type"}</th>
                <th style={"padding:16px"}>{"Winner"}</th>
                <th style={"padding:16px"}>{"Played Against"}</th>
                <th style={"padding:16px"}>{"When Played"}</th>
            </tr>
            {game_rows}
        </table>
    }
}

pub struct PlayerWinsHistory {
    id: String,
    winner: String,
    wins_amount: String
}

fn make_won_by_player_row(ctx: &Context<ScoreBoard>, data: &PlayerWinsHistory) -> Html {
    html!{
        <tr>
            <td style={"padding:8px"}>{data.id.clone()}</td>
            <td style={"padding:8px"}>{data.winner.clone()}</td>
            <td style={"padding:8px"}>{data.wins_amount.clone()}</td>
        </tr>
    }
}

fn make_won_by_player_table(ctx: &Context<ScoreBoard>, datum: Vec<PlayerWinsHistory>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_won_by_player_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"} class={"w3-table w3-striped w3-bordered"}>
            <tr>
                <th style={"padding:16px"}>{"SI. No."}</th>
                <th style={"padding:16px"}>{"Winner or Draw"}</th>
                <th style={"padding:16px"}>{"No. of Wins"}</th>
            </tr>
            {game_rows}
        </table>
    }
}