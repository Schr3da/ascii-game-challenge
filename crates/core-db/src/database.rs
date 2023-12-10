use diesel::prelude::*;

pub struct Database {
    pub path: String,
    pub connection: SqliteConnection,
}

impl Database {
    pub fn new(path: String) -> Self {
        let connection = SqliteConnection::establish(path.as_str())
            .unwrap_or_else(|_| panic!("Error connecting to {}", path));

        Database { path, connection }
    }

    pub fn save(&self) {
        println!("save game state");
    }

    pub fn delete(&self) {
        println!("delete game state");
    }

    pub fn load(&self) {
        println!("load game state");
    }
}
