#![allow(unused_imports)]
use core::{time, num};
use core::result::Result::Ok;
use std::fmt::{Display, Formatter};
use std::{result, error::Error, fmt};
use regex::{Regex};
use anyhow::{anyhow, Result};

use db::{Database};
use rusqlite::{Connection, Rows};
use user::{User};

use crate::user::{remove_user, user_command, DEFAULT_ERROR, CLEAR_CMD};
use crate::db::{add_new_storage_bin};
use crate::warehouse::StorageBin;

mod warehouse;
mod user;
mod db;


fn main(){
    let mut database = Database::initialise_database();

    start_menu(&mut database.connection);
}

fn start_menu(db_connection: &mut Connection){
    println!("Welcome to WarehouseInc!\n");

    let pre_menu_str = 
    String::from("1. Login\n2. Register");

    loop {
        println!("{}", pre_menu_str);
        let cmd_pre_menu = user_command(false).unwrap();

        match cmd_pre_menu.as_str().trim() {
            "1" => {
                let verify_user = User::verify_credentials();
                match verify_user {
                    Ok(user) => {
                        if User::login(user, db_connection) {
                            warehouse_menu(db_connection);
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            "2" => {
                let verify_user = User::verify_credentials();
                match verify_user {
                    Ok(user) => {
                        User::register(user, db_connection).unwrap();
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            _ => {
                println!("Something went wrong!\n");
                continue;
            }
        }
    }
}

fn warehouse_menu(db_connection: &mut Connection){
    let sleep = time::Duration::from_secs(2);
    std::thread::sleep(sleep);
    print!("{}", CLEAR_CMD);

    loop {
        let warehouse_menu_str = 
            String::from("1. Search Storage Bin\n2. Add/Move Stock\n3. Add New Storage Bin\n");
        println!("{}", warehouse_menu_str);   

        let mut cmd = user_command(false).unwrap();

        match cmd.as_str().trim() {
            "1" => {
                println!("Enter Storage Bin Location:"); //  (Format EX: AA 01 S1 01 / AA01S101) | Make a little manual on button press.
                cmd = user_command(false).unwrap();

                match StorageBin::validate_storage_bin(cmd.as_str()) {
                    Ok(storage_bin) => {
                        let _stock = StorageBin::print_stock(&storage_bin, db_connection);

                        cmd = user_command(true).unwrap_or_else(|err|{
                            err.to_string()
                        });
                    } 
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                };
            }
            "2" => {
                // Incomplete Function
                warehouse_add_or_move(db_connection);
                
            }
            "3" => {
                println!("Enter Storage Bin Location:"); //  (Format EX: AA 01 S1 01 / AA01S101) 
                cmd = user_command(false).unwrap();

                match StorageBin::validate_storage_bin(cmd.as_str()) {
                    Ok(storage_bin) => {
                        add_new_storage_bin(db_connection, &storage_bin.location, true).unwrap();
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }

            }
            _ => {
                println!("Something went wrong!\n");
                continue;
            }
        }
    }

}

fn warehouse_add_or_move(db_connection: &mut Connection) -> Result<()>{
    // Incomplete Function - finish.

    println!("Enter Storage Bin Location:");
    let cmd = user_command(false).unwrap();

    match StorageBin::validate_storage_bin(cmd.as_str()) {
        Ok(storage_bin) => {
            let storage_rows = Database::search_row(db_connection, "Location", &storage_bin.location)?;

            match storage_rows {
                Some(storage_items) => {
                    for storage_bin in storage_items{
                        let _stock = StorageBin::print_stock(&storage_bin, db_connection);
                    }
                }
                None => {
                    println!("Location {} doesn't exist!", &storage_bin.location);
                }
            }
            // Implement Quit button to exit at any point.
            let barcode = StorageBin::validate_barcode();
            let material = StorageBin::validate_material();
            println!("Item Added"); // Add necessary values
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    Ok(())

}