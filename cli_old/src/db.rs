use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

diesel_migrations::embed_migrations!("migrations");

pub fn establish_connection() -> SqliteConnection {
    let db_name = std::env::var("DB_NAME").unwrap_or("main".to_string());
    let database_url = format!("file:db/{}.sqlite3", db_name);
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn run_migrations(conn: &SqliteConnection) {
    embedded_migrations::run_with_output(conn, &mut std::io::stdout())
        .expect("Failed to run migrations");
}
