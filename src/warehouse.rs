use std::{collections::HashMap, char, error::Error};
use rusqlite::Connection;
use anyhow::{anyhow, Result};

use crate::Database;
use crate::Regex;
use crate::user::user_command;
use crate::warehouse_menu;


#[derive(Debug)]
pub struct StorageBin{
    pub barcode: Option<u64>,
    pub location: String,
    pub description: Option<String>,
    pub material_number: Option<u64>,
    pub quantity: Option<u64>
}

impl StorageBin {
    fn new(bin_location: String) -> StorageBin {
        StorageBin { 
            barcode: None,
            location: bin_location,
            description: None,
            material_number: None,
            quantity: None,
        }
    }


    fn add_product(&mut self, barcode: u64, quantity: i32){
    }

    fn remove_product(&mut self, barcode: u64, quantity: i32){
    }
    fn move_product(&mut self, barcode: u64, quantity: i32){
        
    }

    pub fn print_stock(&self, db_connection: &mut Connection) -> Result<()>{
        // Grab items from location in DB
        let result = Database::search_row(db_connection, "Location", &self.location)?;

        match result {
            Some(rows) => {
                let mut product_count = 0;
                for row in rows.iter() {
                    if row.barcode != None {
                        product_count+=1;
                    }
                }
                println!("Found {} products in {}!\n", product_count, self.location);
                if product_count >= 1 {
                    for row in rows {
                        println!("Description: {:#?}\nQuantity: {:#?}\nBarcode: {:#?}\nMaterial Number: {:#?}",
                        row.description, row.quantity, row.barcode, row.material_number);    
                    }
                }
            }
            None => {
                println!("Location {} doesn't exist!", self.location);
            }
        }
        Ok(())
    }

    pub fn validate_storage_bin(command:&str ) -> Result<StorageBin>{
        // Ex Data:
        // AA 02 S1 01 || AA02S101 || AA02G01
        // aa 02 s1 01 || aa02s101 || AA 02 G 01

        // Ground Locations
        let rx1 = Regex::new(r"[Aa]+[A-zZ]\s[0-9]+\s[Ss][12]\s0[12]").unwrap();
        let rx2 = Regex::new(r"[Aa]+[A-zZ][0-9]+[Ss][12]0[12]").unwrap();
        // Above S Locations
        let rx3 = Regex::new(r"[Aa]+[A-zZ]\s[0-9]+\s[A-zZ]\s0[12]").unwrap();
        let rx4 = Regex::new(r"[Aa]+[A-zZ][0-9]+[A-zZ]0[12]").unwrap();

        if rx1.is_match(command) || rx2.is_match(command) ||
           rx3.is_match(command) || rx4.is_match(command){
            Ok(StorageBin::new(StorageBin::storage_bin_entry_format(command)))
        } else {
            Err(anyhow!("Invalid storage bin format."))
        }
    }
    
    fn storage_bin_entry_format(command: &str) -> String {
        let mut result_str = String::new();
        let split_spaces =  command.split(" ");
        let regex_vec = vec![r"[Aa]+[A-zZ]", r"0+[0-9]", r"[Ss]+[0-3]", r"^[A-zZ]$"];
        let mut is_upper_location = false;

        // aa01s101 | aa01a01
        if !command.contains(" "){
            for (i, c) in command.chars().enumerate() {
                result_str.push(char::to_uppercase(c).next().unwrap());

                if i == 4 && !matches!(c, 's' | 'S'){
                    result_str.push(' ');
                    is_upper_location = true;
                }

                // Add ' ' very two chars - unless above if statement is called.
                else if (i + 1) % 2 == 0 && i + 1 != command.len() && is_upper_location == false{
                    result_str.push(' ');
                }
            }
            return result_str.trim().to_string();
        }

        // aa 01 s1 01 | aa 01 a 01 
        // Manipulate the original string
        for split in split_spaces {
            for regex in regex_vec.iter() {
                let rgx = Regex::new(regex).unwrap();

                if rgx.is_match(split){
                    result_str.push_str(&split.to_uppercase());
                    result_str.push_str(" ");
                }
            }
        }

        result_str.trim().to_string()
    }


    pub fn validate_barcode() -> Option<u64>{
        // ie.5056242707712 | 13 digits
        println!("Barcode:"); 
        let command = user_command(true).unwrap_or_else(|err|{
            println!("{}\n", err);
            err.to_string() 
        });

        let command_as_u64 = command.trim().parse::<u64>();
        match command_as_u64 {
            Ok(num) => {
                if command.trim().len().eq(&13){
                    Some(num)
                } else {
                    println!("Double check the barcode!");
                    Self::validate_barcode()
                }
            }
            Err(_) => {
                Self::validate_barcode()
            }
        }
    }

    pub fn validate_material() -> Option<u64>{
        // Ie. 1000000002734 | 13 Digits
        let mut material:u64 = 1000000000000;
        let material_len = 13;

        println!("Material Number:"); 
        let command = user_command(true).unwrap_or_else(|err|{
            println!("{}\n", err);
            err.to_string() 
        });
        
        let command_as_u64 = command.trim().parse::<u64>();
        match command_as_u64 {
            Ok(new_material) => {
                if command.trim().len() == material_len{
                    Some(new_material)
                } else if command.trim().len() > 0 && command.trim().len() < material_len{
                    // Allows for short-hand material input 
                    // Ie. 2748 => 1000000002748
                    let num_digits_to_replace = new_material.to_string().len();
                    let temp_material = material / 10u64.pow(num_digits_to_replace.try_into().unwrap());
                    material = temp_material * 10u64.pow(num_digits_to_replace as u32) + new_material;
                    Some(material)
                } else{
                    println!("Double check the material number!");
                    Self::validate_material()
                }
            }
            Err(_) => {
                Self::validate_material()
            }
        }
    }

}