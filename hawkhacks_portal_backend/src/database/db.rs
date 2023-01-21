use diesel::r2d2::{ConnectionManager, Pool};
use diesel::pg::PgConnection;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = "postgres://postgres:postgres@localhost:5432/rust_backend";
    let manager = ConnectionManager::<PgConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(3)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}