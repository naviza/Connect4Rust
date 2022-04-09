#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
use rocket::response::status::NotFound;

mod database_functions;

pub fn parse_leaderboard(board: String) -> Vec<(String, String)> {
	let mut result: Vec<(String, String)> = Vec::new();
	for game in board.split("|") {
		let mut data: Vec<&str> = game.split(",").collect();
		result.push((data[0].to_string(), data[1].to_string()));
	}
	result
}

pub fn parse_computer(score: String) -> Vec<String> {
	score.split("|").map(|x| x.to_string()).collect()
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

#[get("/clearhistory")]
fn clearhistory() -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	
	let mut status: String = String::from("");
	match db.setup_database() {
		Ok(_) => status = status + "Finished database setup",
		Err(e) => return Err(NotFound(e.to_string()))
	};
	Ok(status)
}

#[get("/gamehistory")]
fn gamehistory() -> Result<String, NotFound<String>> {
    let db = database_functions::UserBase {
        fname: String::from("./data/gp3.db"),
    };
    match db.get_games_history() {
        Ok(val) => Ok(val),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[get("/computergames")]
fn computergames() -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
        fname: String::from("./data/gp3.db"),
    };
    match db.computer_games() {
        Ok(val) => Ok(val),
        Err(e) => Err(NotFound(e.to_string())),
    }
}

#[get("/leaderboard")]
fn leaderboard() -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.get_leaderboard() {
		Ok(score) => Ok(score),
		Err(e) => Err(NotFound(e.to_string()))
	}
}

#[get("/computerinfo")]
fn computerinfo() -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.computer_info() {
		Ok(score) => Ok(score),
		Err(e) => Err(NotFound(e.to_string()))
	}
}

#[get("/deleteplayer/<name>/<password>")]
fn deleteplayer(name: String, password: String) -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
        fname: String::from("./data/gp3.db"),
    };
    match db.delete_user(name.trim(), password.trim()) {
		Ok(_) => Ok(String::from("Player removed")),
		Err(_) => Err(NotFound("Could not remove player".to_string()))
	}
}

#[get("/addplayer/<name>/<password>")]
fn addplayer(name: String, password: String) -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.new_user(name.trim(), password.trim()) {
		Ok(_) => Ok(String::from("Player added successfully")),
		Err(_) => Err(NotFound("Player was not added".to_string()))
	}
}

#[get("/login/<name>/<password>")]
fn login(name: String, password: String) -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.login(name.trim(), password.trim()) {
		Ok(result) => if result { Ok("t".to_string()) } else { Ok("f".to_string()) },
		Err(_) => Err(NotFound("Failed to login".to_string()))
	} 
}

#[get("/newgame/<player1>/<player2>/<gametype>/<winner>")]
fn newgame(player1: String, player2: String, gametype: String, winner: String) -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.new_game(player1.trim(), player2.trim(), gametype.trim(), winner.trim()) {
		Ok(_) => Ok(String::from("Game Saved")),
		Err(_) => Err(NotFound("Could not save game".to_string()))
	}
}

#[get("/deletegame/<gameID>")]
fn deletegame(gameID: String) -> Result<String, NotFound<String>> {
	let db = database_functions::UserBase {
		fname: String::from("./data/gp3.db"),
	};
	match db.delete_game(gameID.trim()) {
		Ok(_) => Ok(String::from("Game deleted")),
		Err(_) => Err(NotFound("Could not delete game".to_string()))
	}
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, clearhistory, gamehistory, leaderboard, deleteplayer, addplayer, login, newgame, deletegame])
        .launch();
}
