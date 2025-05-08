use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;


pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}