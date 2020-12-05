#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod command_line;
mod core;
mod error;
mod models;
mod repos;
mod schema;

#[async_std::main]
async fn main() {
    command_line::start_command_line().await
}
