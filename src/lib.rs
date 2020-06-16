use sqlite::{self, Connection, Result};
use std::path::Path;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Connection> {
    let connection = sqlite::open(path).unwrap();
    unsafe {
        sqlite3_sys::sqlite3_enable_load_extension(connection.as_raw(), 1);
    }
    connection
        .execute("SELECT load_extension('mod_spatialite.so');")
        .unwrap();
    unsafe {
        sqlite3_sys::sqlite3_enable_load_extension(connection.as_raw(), 0);
    }
    Ok(connection)
}

#[test]
fn test_open() -> anyhow::Result<()> {
    open("spatialite_rs_test.db")?;
    std::fs::remove_file("spatialite_rs_test.db")?;
    Ok(())
}
