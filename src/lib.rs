use sqlite::*;
use std::path::Path;

pub fn open<T: AsRef<Path>>(path: T) -> Result<Connection> {
    let connection = sqlite::Connection::open(path)?;
    load_spatialite_extension(&connection);
    Ok(connection)
}

pub fn open_with_flags<T: AsRef<Path>>(path: T, flags: OpenFlags) -> Result<Connection> {
    let connection = sqlite::Connection::open_with_flags(&path, flags)?;
    load_spatialite_extension(&connection);
    Ok(connection)
}

fn load_spatialite_extension(conn: &Connection) {
    load_custom_extension(conn, "mod_spatialite.so");
}

fn load_custom_extension(conn: &Connection, module: &str) {
    unsafe {
        sqlite3_sys::sqlite3_enable_load_extension(conn.as_raw(), 1);
    }
    conn.execute(format!("SELECT load_extension('{}');", module))
        .unwrap();
    unsafe {
        sqlite3_sys::sqlite3_enable_load_extension(conn.as_raw(), 0);
    }
}

#[test]
fn test_open() -> anyhow::Result<()> {
    {
        open("spatialite_rs_test.db")?;
    }

    open_with_flags("spatialite_rs_test.db", OpenFlags::new().set_read_only())?;
    std::fs::remove_file("spatialite_rs_test.db")?;
    Ok(())
}
