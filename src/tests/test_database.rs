#[cfg(test)]
pub mod test_database {
    use crate::database::{models, sqlite3};

    fn integration() -> sqlite3::Integration {
        let db_path = String::from("test_game.db");
        let integration = sqlite3::Integration::new(
            db_path
        ).unwrap();

        integration.create_table();
        integration
    }

    fn dummy() -> models::Game {
        models::Game {
            id: None,
            name: String::from("dummy"),
            exe_path: String::from("dummy.exe"),
        }
    }


    #[test]
    fn test_add_game() {
        let integration = integration();
        let game = dummy();

        integration.add_game(game);

        let games = integration.list_all();
        assert_eq!(games.len(), 1);
    }

    #[test]
    fn test_delete_game() {
        let integration = integration();
        let id = String::from("1");

        integration.delete_game(id);

        let games = integration.list_all();
        assert_eq!(games.len(), 0);
    }
}