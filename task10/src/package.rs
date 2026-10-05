use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

const PACKAGE_FILE: &str = "assets/package.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

pub fn get_packages() -> Result<Vec<Package>, Box<dyn Error>> {
    let file = fs::read_to_string(PACKAGE_FILE).unwrap_or_else(|_| "[]".to_string());
    let packages = serde_json::from_str(&file)?;

    Ok(packages)
}

pub fn install(package: Package) -> Result<(), Box<dyn Error>> {
    let mut packages = get_packages()?;

    if packages.iter().any(|item| item.name == package.name) {
        println!("Уже имеется пакет: {}", package.name);
        return Ok(());
    }

    println!("Добавлен пакет: {} {}", package.name, package.version);
    packages.push(package);
    save_packages(&packages)
}

pub fn remove(package_name: &str) -> Result<(), Box<dyn Error>> {
    let mut packages = get_packages()?;
    let old_len = packages.len();

    packages.retain(|package| package.name != package_name);

    if packages.len() == old_len {
        println!("Не удалось найти пакет: {}", package_name);
        return Ok(());
    }

    println!("Удален пакет: {}", package_name);
    save_packages(&packages)
}

fn save_packages(packages: &[Package]) -> Result<(), Box<dyn Error>> {
    let updated = serde_json::to_string_pretty(packages)?;
    fs::write(PACKAGE_FILE, updated)?;

    Ok(())
}
