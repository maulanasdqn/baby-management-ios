use rusqlite::{Connection, Result as SqlResult};
use std::sync::{Arc, Mutex};
pub type Pool = Arc<Mutex<Connection>>;
pub fn open(db_path: &str) -> SqlResult<Pool> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch(include_str!("migrations/0001_init.sql"))?;
    conn.execute_batch(include_str!("migrations/0002_activities.sql"))?;
    Ok(Arc::new(Mutex::new(conn)))
}
pub fn open_in_memory() -> SqlResult<Pool> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(include_str!("migrations/0001_init.sql"))?;
    conn.execute_batch(include_str!("migrations/0002_activities.sql"))?;
    Ok(Arc::new(Mutex::new(conn)))
}
