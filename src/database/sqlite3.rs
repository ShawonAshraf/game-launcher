use super::models::Game;
use sqlite;
use sqlite::State;

/// Represents an integration with a SQLite3 database.
///
/// This struct encapsulates the connection to a SQLite3 database, allowing
/// operations related to interacting with the database.
pub struct Integration {
    db_conn: sqlite::Connection,
}

impl Integration {
    /// Creates a new instance of the `Integration` struct by opening a SQLite database connection.
    ///
    /// # Arguments
    ///
    /// * `db_name` - A string slice that holds the name or path to the SQLite database file.
    ///
    /// # Returns
    ///
    /// A `Result` containing either an `Integration` instance with a valid database connection or a static string error message if the connection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string());
    /// match integration {
    ///     Ok(integration) => println!("Database connection established"),
    ///     Err(e) => println!("Failed to establish database connection: {}", e),
    /// }
    /// ```
    pub fn new(db_name: String) -> Result<Integration, &'static str> {
        let connection = sqlite::Connection::open(db_name).expect("Connection should be established");
        Ok(Integration { db_conn: connection })
    }

    /// Creates a table named `game` in the SQLite database if it does not already exist.
    ///
    /// The table has three columns:
    /// - `id`: An integer that serves as the primary key.
    /// - `name`: A text field to store the name of the game.
    /// - `exe_path`: A text field to store the path to the executable file of the game.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// integration.create_table();
    /// println!("Table created successfully");
    /// ```
    pub fn create_table(&self) {
        let init_query = "create table if not exists game (id integer primary key, name text, exe_path text)";
        self.db_conn.execute(init_query).unwrap();
    }


    /// Inserts a new game into the `game` table.
    ///
    /// # Arguments
    ///
    /// * `game` - A `Game` struct containing the name and executable path of the game to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    /// use crate::database::models::Game;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// let new_game = Game { name: "MyGame".to_string(), exe_path: "/path/to/game.exe".to_string() };
    /// integration.add_game(new_game);
    /// println!("Game added successfully");
    /// ```
    pub fn add_game(&self, game: Game) {
        let query = format!("insert into game(name, exe_path) values('{}', '{}')",
                            &game.name, &game.exe_path);
        self.db_conn.execute(query).unwrap();
    }


    /// Deletes a game from the `game` table by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the ID of the game to be deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// integration.delete_game("1".to_string());
    /// println!("Game deleted successfully");
    /// ```
    pub fn delete_game(&self, id: String) {
        let query = format!("delete from game where id='{}'", id);
        self.db_conn.execute(query).unwrap();
    }


    /// Retrieves all games from the `game` table.
    ///
    /// # Returns
    ///
    /// A vector of `Game` structs representing all the games in the database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    /// use crate::database::models::Game;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// let games = integration.list_all();
    /// for game in games {
    ///     println!("ID: {}, Name: {}, Exe Path: {}", game.id, game.name, game.exe_path);
    /// }
    /// ```
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
