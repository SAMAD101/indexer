use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    if let Ok(database_url) = env::var("DATABASE_URL") {
        let mut conn = PgConnection::establish(&database_url)
            .expect("Unable to establish connection to database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    }
}