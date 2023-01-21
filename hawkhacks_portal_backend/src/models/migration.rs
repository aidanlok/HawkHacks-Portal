use diesel::pg::{PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// use super::message::{Message, NewMessage};
// use super::frame::{Frame, FrameFilter, InsertFrame};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}