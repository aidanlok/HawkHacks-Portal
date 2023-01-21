use simple_logger::SimpleLogger;

use custom_types::{PGConnPool, Users};

use crate::database::{db};
use crate::models::{migration};

mod custom_types;
mod security_questions;
mod schema;
mod database;
mod models;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let pool: PGConnPool = db::get_connection_pool();
    let users: Users = Users::default();
    run_diesel_migrations(pool.clone());
    
    tokio::join!(
        webserver::start_webserver(pool.clone(), users.clone()),
    );
}

fn run_diesel_migrations(pool: PGConnPool) {
    let mut pool_get = pool.get();
    let conn = match &mut pool_get {
        Result::Ok(v) => { v },
        Result::Err(e) => { log::error!("pool conn get err: {e}"); return; }
    };
    migration::run_migrations(conn);
}