use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct MarkDownStatic {
    count_words: u64,
    count_titles: u64,
    count_links: u64,
    count_codeblocks: u64,
}
// cargo run -- ERROR app.log
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mkdown_statistic = MarkDownStatic {
        count_words: 0,
        count_titles: 0,
        count_links: 0,
        count_codeblocks: 0,
    };
    let path = PathBuf::from("files").join("mark.md");
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let mut is_code_block = false;

    for read_line in buffer.lines() {
        let read_line = read_line?;
        mkdown_statistic.count_words += read_line.split_whitespace().count() as u64;
        if read_line.trim_start().starts_with('#') {
            mkdown_statistic.count_titles += 1;
        }
        if read_line.contains("://") {
            mkdown_statistic.count_links += 1;
        }

        if read_line.starts_with("```") && is_code_block {
            is_code_block = false;
            continue;
        }
        if read_line.starts_with("```") && !is_code_block {
            mkdown_statistic.count_codeblocks += 1;
            is_code_block = true;
        }
        
    }
    println!("Слов: {}", mkdown_statistic.count_words);
    println!("Заголовков: {}", mkdown_statistic.count_titles);
    println!("Ссылок: {}", mkdown_statistic.count_links);
    println!("Код-блоков: {}", mkdown_statistic.count_codeblocks);

    Ok(())
}

