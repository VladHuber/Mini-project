use regex::Regex;

enum WrongPass {
    Digit, LetterUpper, Words, Symbols
}
fn main() {
    let mut wrong_pass: Vec<WrongPass> = Vec::new();
    let words = ["password", "12345","qwerty"];
    let pass = String::from("password123");

    if pass.len() < 8{
        println!("Пароль введен меньше 7 символов");
    }
    // let re = Regex::new(r"^([0-9a-z]{8,})$").unwrap();
    let digit_pattern = Regex::new(r"\d").unwrap();
    let letter_uppercase_pattern = Regex::new(r"[A-Z]").unwrap();
    let letter_lowercase_pattern = Regex::new(r"[a-z]").unwrap();

    let allowed_specials = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let has_allowed_special = pass.chars().any(|c| allowed_specials.contains(c));
    if !has_allowed_special{
        wrong_pass.push(WrongPass::Symbols);
    }
    if !digit_pattern.is_match(&pass){
        wrong_pass.push(WrongPass::Digit);
    }
    if !letter_uppercase_pattern.is_match(&pass) || !letter_lowercase_pattern.is_match(&pass){
        wrong_pass.push(WrongPass::LetterUpper);
    }
    
    for word in words{
        if pass.contains(word){
            wrong_pass.push(WrongPass::Words);
            break;
        }
    }
    if wrong_pass.len() > 0{
        println!("Слабый пароль");
        println!("Причины:");
        for wrong_p in wrong_pass {
            match wrong_p{
                WrongPass::Digit => println!("- нет цифр"),
                WrongPass::LetterUpper => println!("- нет букв, возможно прописных"),
                WrongPass::Words => println!("- есть словарное слово"),
                WrongPass::Symbols => println!("- нет спецсимволов")
            }
        }
    }else {
        println!("Сильный пароль");
    }
}
