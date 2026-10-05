use chrono::Datelike;
use chrono::{NaiveDate, Local};
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs;
use std::io;

#[derive(Deserialize, Serialize, Debug)]
struct Cost {
    amount: i64,
    category: String,
    date: String,
}

fn main() -> std::io::Result<()> {
    loop {
        println!("Введите команду: ");
        println!("add, stats, top, mount ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        match command.trim() {
            "add" => {
                println!("Amount:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                println!("Category:");
                let mut category = String::new();
                io::stdin().read_line(&mut category).unwrap();
                println!("Date:");
                let mut date = String::new();
                io::stdin().read_line(&mut date).unwrap();
                if add_cost(Cost {
                    amount: amount.parse::<i64>().unwrap_or(0),
                    category: category,
                    date: date,
                })
                .is_ok()
                {
                    println!("Товар добавлен");
                }
            }
            "stats" => {
                let _ = stats();
            }
            "mount" => {
                get_mount();
            }
            "top" => {
                get_top();
            }
            _ => {
                break;
            }
        }
    }

    Ok(())
}

fn add_cost(cost: Cost) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("assets/manager.json").unwrap_or("[]".to_string());
    let mut expenses: Vec<Cost> = serde_json::from_str(&file).unwrap();
    expenses.push(cost);
    let updated = serde_json::to_string_pretty(&expenses).unwrap();
    fs::write("assets/manager.json", updated).unwrap();

    Ok(())
}
fn stats() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("assets/manager.json").unwrap_or("[]".to_string());
    let expenses: Vec<Cost> = serde_json::from_str(&file).unwrap();
    for expense in expenses {
        println!(
            "{} - {} - {} ",
            expense.amount, expense.category, expense.date
        );
    }
    Ok(())
}
fn get_mount() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("assets/manager.json").unwrap_or("[]".to_string());
    let expenses: Vec<Cost> = serde_json::from_str(&file)?;
    for expense in expenses {
        let native = NaiveDate::parse_from_str(&expense.date, "%Y-%m-%d")?;
        let now = Local::now().date_naive();

        if native.year() == now.year() && native.month() == now.month() {
        println!(
            "{} - {} - {} ",
            expense.amount, expense.category, expense.date
        );
        }
       
    }
    Ok(())
}
fn get_top() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("assets/manager.json").unwrap_or("[]".to_string());
    let mut expenses: Vec<Cost> = serde_json::from_str(&file)?;
    expenses.sort_by(|a, b| b.amount.cmp(&a.amount));
    for expense in expenses {
        println!(
            "{} - {} - {} ",
            expense.amount, expense.category, expense.date
        );
        
       
    }
    Ok(())
}
/*
add
stats
month
top
*/
