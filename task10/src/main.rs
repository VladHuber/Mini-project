mod package;

use package::{Package, get_packages, install, remove};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: cargo run -- <Команда>");
        return Ok(());
    }
    let command = &args[1];
    match command.trim() {
        "list" => {
            let packages = get_packages()?;
            for pack in packages {
                println!("{} {}", pack.name, pack.version);
            }
        }
        "install" => {
            if args.len() < 4 {
                eprintln!("Использование: cargo run -- install <name> <version>");
                return Ok(());
            }
            install(Package {
                name: args[2].clone(),
                version: args[3].clone(),
            })?;
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Использование: cargo run -- remove <name>");
                return Ok(());
            }
            remove(&args[2])?;
        }
        _ => eprintln!("Неизвестная команда: {}", command),
    }

    Ok(())
}
