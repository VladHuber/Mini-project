use std::fs::{self,File};
use std::collections::{HashMap};
use std::path::PathBuf;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    for entry in fs::read_dir(PathBuf::from("assets/"))?{
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let file_hash = shafile(entry.path())?;
        files.entry(file_hash).or_default().push(file_name);
    }
    let duplicate_files = find_duplicates(files);
    println!("{:#?}", duplicate_files);
    Ok(())
}

fn find_duplicates(hashes: HashMap<String,Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut duplicates = HashMap::new(); // HashSet исключит повторы среди самих дубликатов

    for (key,item) in hashes {
        if item.len() > 1 {
            duplicates.insert(key, item);
        }
    }

    duplicates
}

fn shafile(path: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(path).unwrap();
    let mut buffer = [0; 1024];
    let mut hasher = blake3::Hasher::new();
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_string())
}
