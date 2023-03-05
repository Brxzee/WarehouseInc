use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::Connection;
use std::{io, fmt::{format, Error}, char::ToLowercase, collections::HashMap};

use crate::{user};
use anyhow::{anyhow, Result};

pub static DEFAULT_ERROR: &str = "Something went wrong!";
pub static CLEAR_CMD: &str = "\x1B[2J\x1B[1;1H";


#[derive(Debug)]
pub struct User{
    pub username: String,
    pub password: String,
    //is_superuser: bool,
}

impl User {
    pub fn verify_credentials() -> Result<Self, anyhow::Error>{
        println!("Enter Username:");
        let username = user_command(true).unwrap_or_else(|err|{
            err.to_string() 
        });
        println!("Enter Password:");
        let password = user_command(true).unwrap_or_else(|err|{
            err.to_string()
        });
        if username.eq(DEFAULT_ERROR) || password.eq(DEFAULT_ERROR){
            return Err(anyhow!("{}",DEFAULT_ERROR));
        } 
        let user = Self {
            username: username,
            password: password,
        };

        Ok(user)
    }

    pub fn register(self, db_connection: &mut Connection) -> Result<(), anyhow::Error> {
        let db_query = "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?)";
        let mut db_statement = db_connection.prepare(db_query)?;
    
        let username_exists = if let Some(exists) = db_statement.query_row([&self.username], |row| row.get(0)).ok() {
            exists
        } else {
            return Err(anyhow!(DEFAULT_ERROR));
        };
    
        if username_exists {
            println!("Username {} already exists!", self.username);
            return Ok(());
        }
    
        let hashed_pass = hash_password(&self.password)?;
    
        let user = User {username: self.username.to_owned(), password: hashed_pass}; // Store in DB
    
        let db_query = "INSERT INTO users VALUES (?, ?)";
        db_connection.execute(db_query, [&user.username, &user.password])?;
    
        println!("User {} has been added to WarehouseInc", user.username);
        Ok(())
    }

    pub fn login(self, db_connection: &mut Connection) -> bool {
        // Add 10 min inactivity log out.
        let mut success: bool = false;
    
        let db_query = format!("SELECT * FROM users WHERE username LIKE '{}'", self.username);
        let mut statement = db_connection.prepare(db_query.as_str()).unwrap();
        
        let username_exists = statement.exists([]);
        if username_exists.unwrap().eq(&false){
            println!("Username {} doesn't exist!", self.username);
            return false;
        }
        
        let user_iter = statement.query_map([], |row| {
            Ok(User {
                username: row.get(0)?,
                password: row.get(1)?,
            })
        }).unwrap();
    
        for user in user_iter {
            if user.as_ref().unwrap().username.eq(&self.username){
                if verify(&self.password, user.unwrap().password.as_str()).unwrap(){
                    println!("\nYou've been granted entry to WarehouseInc\n");
                    success = true;
                } else {
                    println!("Password is incorrect!");
                }
            }
        }
    
        if success.eq(&true){
            return true;
        }
        else {
            return false;
        }
    }
}

pub fn user_command(require_result: bool) -> Result<String>{
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    command = command.trim().to_owned();

    if !require_result {
        return Ok(command)
    };

    match command.is_empty() {
        true => Err(anyhow!("Something went wrong!")),
        false => Ok(command),
    }
}


fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn remove_user(db_connection: &mut Connection){
    // Fix Function once superusers are implemented.
    println!("Enter username to remove:");
    let username = user_command(true).unwrap_or_else(|err|{
        println!("{}", err);
        err.to_string() 
    });

    let mut db_query = format!("SELECT * FROM users WHERE username LIKE '{}'", username);
    let mut statement = db_connection.prepare(db_query.as_str()).unwrap();

    let username_exists = statement.exists([]);
    if username_exists.unwrap().eq(&false){
        println!("Username {} doesn't exist!", username);
        return;
    }

    let user_iter = statement.query_map([], |row| {
        Ok(User {
            username: row.get(0)?,
            password: row.get(1)?,
        })
    }).unwrap();

    for user in user_iter {
        if user.unwrap().username.eq(username.as_str()){
            println!("Are you sure you want to remove user {} from WarehouseInc? : (Y/N)", username);
            let verify_removal = user_command(true).unwrap_or_else(|err|{
                println!("{}", err);
                err.to_string() 
            });

            match verify_removal.as_str().trim() {
                "Y" | "y" => {
                    //Remove from database
                    db_query = format!("DELETE FROM users WHERE username='{}'", username);
                    db_connection.execute(&db_query, ()).unwrap();
                    println!("User {} has been removed!", username);
                }
                "N" | "n" => {
                    return;
                }
                _ => {
                    println!("Invalid Input!\n");
                }
            }
        } else {
            println!("User {} doesn't exist!", username);
        }
    }
}

