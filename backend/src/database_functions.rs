use argon2::{self, Config};
use sqlite::Error as SqErr;
use sqlite::State;

#[derive(Debug)]
pub enum UBaseErr {
    DbErr(SqErr),
    HashError(argon2::Error),
}

impl From<SqErr> for UBaseErr {
    fn from(s: SqErr) -> Self {
        UBaseErr::DbErr(s)
    }
}

impl From<argon2::Error> for UBaseErr {
    fn from(a: argon2::Error) -> Self {
        UBaseErr::HashError(a)
    }
}

pub struct UserBase {
    pub fname: String,
}

impl UserBase {	
	pub fn setup_database(&self) -> Result<(), SqErr> {
        let conn = sqlite::open(&self.fname)?;
		let mut st = conn.prepare("DELETE FROM Games;")?;
        st.next()?;
		st = conn.prepare("DELETE FROM Players;")?;
        st.next()?;

		let conn = sqlite::open(&self.fname)?;
		match &self.new_user("Computer", "0000") {
			Ok(_) => Ok(()),
			Err(e) => panic!("Could not add computer to database")
		}
	}

    pub fn computer_games(&self) -> Result<String, SqErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("Select * From Games Where won == \"Computer\";")?;

        let mut games_string: String = String::from("");
        while let State::Row = st.next().unwrap() {
            games_string = games_string + st.read::<String>(0).unwrap().trim() + "," 
            				+ st.read::<String>(1).unwrap().trim() + ","
            				+ st.read::<String>(2).unwrap().trim() + ","
            				+ st.read::<String>(3).unwrap().trim() + ","
            				+ st.read::<String>(4).unwrap().trim() + ","
            				+ st.read::<String>(5).unwrap().trim() + "|";
        }

        Ok(games_string)
    }

    pub fn computer_info(&self) -> Result<String, SqErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("Select player1Name, player2Name, won From Games;")?;
        
        let mut wins: i32 = 0;
        let mut against: i32 = 0;
        let mut total: i32 = 0;
        while let State::Row = st.next().unwrap() {
            total = total + 1;
            if st.read::<String>(2).unwrap().eq("Computer") { wins = wins + 1; } 
            if st.read::<String>(0).unwrap().eq("Computer") || st.read::<String>(1).unwrap().eq("Computer") { against = against + 1; }
        }
        Ok(total.to_string() + "|" + against.to_string().trim() + "|" + wins.to_string().trim())
    }
	
    pub fn new_user(&self, username: &str, password: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let salt = b"randomsalt";
        let hpass = argon2::hash_encoded(password.as_bytes(), salt, &Config::default())?;

        let mut st = conn.prepare("insert into Players (playerName, pswd) values (?, ?);")?;
        st.bind(1, username)?;
        st.bind(2, &hpass as &str)?;
        st.next()?;
        Ok(())
    }

    pub fn login(&self, username: &str, password: &str) -> Result<bool, UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("select pswd from users where playerName == ?;")?;
        st.bind(1, username)?;
        match st.next() {
            Ok(_) => {
                let hash = st.read::<String>(0).unwrap();
                match argon2::verify_encoded(&hash.trim(), password.as_bytes()) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(UBaseErr::from(e)),
                }
            }
            Err(e) => Err(UBaseErr::from(e)),
        }
    }

    pub fn delete_user(&self, username: &str, password: &str) -> Result<(), UBaseErr> {
        match &self.login(username, password) {
            Ok(state) => {
                if !state {
                    panic!("Incorrect password for user: {}", username)
                }
            }
            Err(e) => panic!("Could not check password for user: {}", username),
        };

        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("delete from Players where playerName == ?;")?;
        st.bind(1, username)?;
        st.next()?;
        Ok(())
    }

    // Get game history
    pub fn get_games_history(&self) -> Result<String, SqErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("SELECT * from Games;")?;
        
        let mut games_string: String = String::from("");
        while let State::Row = st.next().unwrap() {
            games_string = games_string + st.read::<String>(0).unwrap().trim() + "," 
            				+ st.read::<String>(1).unwrap().trim() + ","
            				+ st.read::<String>(2).unwrap().trim() + ","
            				+ st.read::<String>(3).unwrap().trim() + ","
            				+ st.read::<String>(4).unwrap().trim() + ","
            				+ st.read::<String>(5).unwrap().trim() + "|";
        }

        Ok(games_string)
    }
    
    pub fn get_leaderboard(&self) -> Result<String, SqErr> {
		let conn = sqlite::open(&self.fname)?;
		let mut st = conn.prepare("Select won From Games Order By won ASC;")?;
		
		let mut winner: String = String::from("");
		let mut score = Vec::new();
		while let State::Row = st.next().unwrap() {
			if !winner.eq(st.read::<String>(0).unwrap().trim()) {
				score.push((st.read::<String>(0).unwrap(), 1));
                winner = st.read::<String>(0).unwrap();
			} else {
                let len = score.len() - 1;
				score[len].1 = score[len].1 + 1;
			}
		}
		Ok(score.into_iter().fold(String::from(""), |x, y| x + y.0.trim() + "," + y.1.to_string().trim() + "|"))
	}
    
    pub fn new_game(&self, player_one: &str, player_two: &str, game_type: &str, winner: &str) -> Result<(), UBaseErr> {
		let conn = sqlite::open(&self.fname)?;
		let mut st = conn.prepare("insert into games(player1Name, player2Name, gameType, won, gameDateTime) values (?, ?, ?, ?, CURRENT_TIMESTAMP);")?;
		st.bind(1, player_one)?;
		st.bind(2, player_two)?;
		st.bind(3, game_type)?;
		st.bind(4, winner)?;
        st.next()?;
		Ok(())
	}

    pub fn delete_game(&self, game_id: &str) -> Result<(), UBaseErr> {
        let conn = sqlite::open(&self.fname)?;
        let mut st = conn.prepare("delete from games where gameID == ?;")?;
        st.bind(1, game_id)?;
        st.next()?;
        Ok(())
    }
}
