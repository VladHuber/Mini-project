use std::fs::File;
use std::io::{BufRead, BufReader};

struct CodeReport {
    total_enums: usize,
    total_structs: usize,
    total_funcs: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./project/test.rs")?;
    let reader = BufReader::new(file);

    let mut report = CodeReport {
        total_enums: 0,
        total_structs: 0,
        total_funcs: 0,
    };

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let line = line.trim_start();

        if line.starts_with("enum ") {
            report.total_enums += 1;
            println!("enum at line {}", index + 1);
        } else if line.starts_with("struct ") {
            report.total_structs += 1;
            println!("struct at line {}", index + 1);
        } else if line.starts_with("fn ") {
            report.total_funcs += 1;
            println!("fn at line {}", index + 1);
        }
    }

    println!("Functions: {}", report.total_funcs);
    println!("Structs: {}", report.total_structs);
    println!("Enums: {}", report.total_enums);

    Ok(())
}