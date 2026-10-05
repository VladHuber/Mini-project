use std::{collections::HashMap, fs, path::Path};
use std::error::Error;
fn main() {
    let mut files : HashMap<String, u64> = HashMap::new();
    match read_files(Path::new("D:/Download"), &mut files){
        Ok(files) => files,
        Err(err ) => panic!("Ошибка при открытии файла: {}", err)
    };

    let mut top_files:Vec<(&String, &u64)> = files.iter().collect();
    top_files.sort_by(|a, b| b.1.cmp(a.1));
    let result: Vec<(&String, &u64)> = top_files.into_iter().take(10).collect();
    for (key, val) in result{
        println!("{} {}", key, size_file(*val));
    }

}

fn read_files(dir: &Path, hash_map: &mut HashMap<String,u64>) -> Result<(), Box<dyn Error>>{
    for entry in fs::read_dir(dir)?{
        let entry = entry?;
        let dir_entry = entry;
        let file_name = dir_entry.file_name();
        let file_type = &dir_entry.file_type()?;
        let dir = dir_entry.path(); 
        if file_type.is_dir(){
            read_files(&dir, hash_map)?;
        }else{
            let metadata = fs::metadata(dir)?;
            hash_map.insert(file_name.to_str().unwrap().to_string(), metadata.len());
        }
        
    }
    Ok(())
}

fn size_file(base_size: u64) -> String{
    if base_size >= 1000000000{
        return format!("{:.2}{}", base_size as f64 / 1000000000.0, "GB");
    }
    if base_size >= 1000000{
        return format!("{:.2}{}", base_size as f64 / 1000000.0, "MB");
    }
    if base_size >= 1000{
        return format!("{:.2}{}", base_size as f64 / 1000.0, "KB");
    }
    return format!("{:.2}{}", base_size as f64 / 1.0, "B");
}