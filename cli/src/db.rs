//! Database helpers
use crate::prelude::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

diesel_migrations::embed_migrations!("migrations");

/// Establish connection to sqlite database
pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or("db/main.db".to_string());
    log::debug!("Establishing connection to database {}", database_url);
    Ok(SqliteConnection::establish(&database_url)?)
}

/// Run pending migrations
pub fn run_migrations(conn: &SqliteConnection) -> Result<()> {
    log::debug!("Running migrations");
    Ok(embedded_migrations::run_with_output(
        conn,
        &mut std::io::stdout(),
    )?)
}
