use crate::classes::*;


pub fn get_user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{prompt}");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}


pub fn get_gender_from_string(input: &str) -> Gender {
    match input {
        "m" => {return Gender::MALE;},
        "f" => {return Gender::FEMALE;},
        _ => {return Gender::MALE;}
    };
}