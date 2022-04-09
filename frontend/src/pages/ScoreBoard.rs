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
                id: "id0".to_string(),
                humanplayer: "human".to_string(),
                gametype: "TOOT/OTTA".to_string(),
                winner: "Computer".to_string(),
                whenplayed: "fdsghgjjthre".to_string(),
            }
        });
        let computer_history_table = make_won_by_computer_table(&ctx, comp_history);

        
        let mut player_wins = Vec::new();
        player_wins.push(
            PlayerWinsHistory {
                id: "id1".to_string(),
                winner: "winner1".to_string(),
                wins_amount: "420,69".to_string(),
            }
        );
        player_wins.push(
            PlayerWinsHistory {
                id: "id1".to_string(),
                winner: "winner1".to_string(),
                wins_amount: "420,69".to_string(),
            }
        );
        let player_wins_table = make_won_by_player_table(&ctx, player_wins);


        html! { 
            <>
            
                <h1>{"SCORE BOARD"}</h1>

                <h1>{"Games Won by Computer"}</h1>
                {make_games_won_by_computer_summary(&ctx, 12, 10, 8)}

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
        <table border={"1"}>
            <tr>
                <th>{"Total Games Played	"}</th>
                <th>{"Games Against Computer"}</th>
                <th>{"Games Computer Won"}</th>
            </tr>
            <tr>
                <td>{format!("{}", total_games)}</td>
                <td>{format!("{}", games_aginst)}</td>
                <td>{format!("{}", games_won)}</td>
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
            <td>{data.id.clone()}</td>
            <td>{data.gametype.clone()}</td>
            <td>{data.winner.clone()}</td>
            <td>{data.humanplayer.clone()}</td>
            <td>{data.whenplayed.clone()}</td>
        </tr>
    }
}

fn make_won_by_computer_table(ctx: &Context<ScoreBoard>, datum: Vec<ComputerWinGameData>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_won_by_computer_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"}>
            <tr>
                <th>{"SI. No."}</th>
                <th>{"Game Type"}</th>
                <th>{"Winner"}</th>
                <th>{"Played Against"}</th>
                <th>{"When Played"}</th>
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
            <td>{data.id.clone()}</td>
            <td>{data.winner.clone()}</td>
            <td>{data.wins_amount.clone()}</td>
        </tr>
    }
}

fn make_won_by_player_table(ctx: &Context<ScoreBoard>, datum: Vec<PlayerWinsHistory>) -> Html {
    let mut game_rows = Vec::new();
    for i in 0..datum.len(){
        game_rows.push(make_won_by_player_row(&ctx, &datum[i]));
    }
    html! {
        <table border={"1"}>
            <tr>
                <th>{"SI. No."}</th>
                <th>{"Winner or Draw"}</th>
                <th>{"No. of Wins"}</th>
            </tr>
            {game_rows}
        </table>
    }
}