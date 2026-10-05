use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// cargo run -- ERROR app.log
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Использование: cargo run -- <текст> <файл>");
        return Ok(());
    }
    let path = PathBuf::from("assets").join(&args[2]);
    let find = &args[1];
    dbg!(&path);
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    for read_line in buffer.lines() {
        let read_line = read_line?;
        if read_line.contains(find){
            println!("{}", read_line);
        }
    }
    
    Ok(())
}
