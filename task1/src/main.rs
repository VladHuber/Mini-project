use std::{collections::HashMap, fs};

fn main() {
    // --snip--
    let file_path = "assets/log.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut stats: HashMap<String, u32> = HashMap::new();
    let mut errors: HashMap<String, u32> = HashMap::new();
    for item in contents.lines() {
        if let Some((level, message)) = item.split_once(' ') {
            *stats.entry(level.to_string()).or_insert(0) += 1;

            if level == "ERROR" {
                *errors.entry(message.to_string()).or_insert(0) += 1;
            }
        }
    }
    for (stat, count) in stats {
        println!("{}:{}", stat, count);
    }
    match errors.iter().max_by_key(|(_, count)| *count) {
        Some((message, count)) => {
            println!("Самая частая ошибка:");
            println!("{} ({})", message, count);
        }
        None => println!("Ошибок нет"),
    }
}
