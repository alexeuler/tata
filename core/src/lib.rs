#[macro_use]
extern crate diesel_migrations;

mod db;

pub fn start() {
	let conn = db::establish_connection();
	db::run_migrations(&conn);
}

