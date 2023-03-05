use std::result::Result::Ok;
//use anyhow::Ok;
use rusqlite::{Connection, Result, params, Rows};

use crate::StorageBin;

pub struct Database {
    pub connection: Connection
}

impl Database {
    pub fn initialise_database() -> Database{
        let mut db = Database {
            connection: Connection::open("./users.db").unwrap()
        };

        Self::create_tables(&mut db);
        return db;
    }

    fn create_tables(&mut self){
        Database::create_user_tbl(&mut self.connection);
        Database::create_warehouse_stock_tbl(&mut self.connection);
    }
    fn create_user_tbl(connection: &mut Connection){
        let query = "CREATE TABLE IF NOT EXISTS Users (Username VARCHAR(40), Password VARCHAR(80))";
        connection.execute(query, ()).unwrap();
    }
    fn create_warehouse_stock_tbl(connection: &mut Connection){
        let query = "CREATE TABLE IF NOT EXISTS Warehouse_Stock (Barcode BIGINT, Location VARCHAR(11), Description TEXT, Material_Number BIGINT, Quantity INT32)";
        connection.execute(query, ()).unwrap();
    }

    pub fn search_row(connection: &mut Connection, column_name: &str, search_value: &str) -> Result<Option<Vec<StorageBin>>>{
        let query = format!("SELECT * FROM Warehouse_Stock WHERE {} = ?", column_name);

        let row = connection.query_row(&query,&[search_value.trim()], |row|{
            Ok(StorageBin {
                barcode: row.get(0).ok(),
                location: row.get(1)?,
                description: row.get(2).ok(),
                material_number: row.get(3).ok(),
                quantity: row.get(4).ok(),
            })
        });

        match row {
            Ok(row) => Ok(Some(vec!(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }

}


pub fn add_new_storage_bin(connection: &mut Connection, storage_location: &str, new_location: bool) -> Result<()>{
    let row_exists = Database::search_row(connection, "Location", storage_location)?.is_some();

    if row_exists && new_location {
        println!("Location {} already exists!\n", storage_location);
    } else {
        connection.execute(
            "INSERT INTO Warehouse_Stock (Barcode, Location, Description, Material_Number, Quantity)
             VALUES (?, ?, ?, ?, ?)",
             params![None::<i64>, storage_location, None::<&str>, None::<i64>, None::<i32>]
        )?;
        println!("Storage Added: [{}]\n", storage_location);
    }

    Ok(())
}
