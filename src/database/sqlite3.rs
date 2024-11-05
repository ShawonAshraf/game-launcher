use super::models::Game;
use sqlite;
use sqlite::State;

pub struct Integration {
    db_conn: sqlite::Connection,
}

impl Integration {
    pub fn new(db_name: String) -> Result<Integration, &'static str> {
        let connection = sqlite::Connection::open(db_name).expect("Connection should be established");
        Ok(Integration { db_conn: connection })
    }
    pub fn create_table(&self) {
        let init_query = "create table if not exists game (id integer primary key, name text, exe_path text)";
        self.db_conn.execute(init_query).unwrap();
    }

    pub fn add_game(&self, game: Game) {
        let query = format!("insert into game(name, exe_path) values('{}', '{}')",
                            &game.name, &game.exe_path);
        self.db_conn.execute(query).unwrap();
    }

    pub fn delete_game(&self, id: String) {
        let query = format!("delete from game where id='{id}'");
        self.db_conn.execute(query).unwrap();
    }


    pub fn list_all(&self) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let query = "select * from game";
        let mut statement = self.db_conn.prepare(query).unwrap();


        while let Ok(State::Row) = statement.next() {
            let id = Some(statement.read::<String, _>("id").unwrap());
            let name = statement.read::<String, _>("name").unwrap();
            let exe_path = statement.read::<String, _>("exe_path").unwrap();

            games.push(Game { id, name, exe_path });
        }


        games
    }
}
