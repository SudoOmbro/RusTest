/// test program
pub mod classes;
pub mod commands;
pub mod utils;
pub mod random_utils;
pub mod csvutils;

use classes::*;
use commands::*;
use utils::*;


fn main() {
    let mut jobs: [String; JOBS_NUM] = Default::default();
    let mut context: Context = Context { last_id: 0, people: vec![] };
    for i in 0usize..JOBS_NUM {
        let input_string = get_user_input(format!("job {}: ", i+1).as_str());
        jobs[i] = input_string;
    }
    loop {
        let command_string: String = get_user_input("command: ");
        let command_args: Vec<&str> = command_string.split(" ").collect();
        let args = command_args[1..].to_vec();
        match command_args[0] {
            "exit" => break,
            "add" => add_person(&mut context, args),
            "del" => remove_person(&mut context, args),
            "rank" => rank_people(&context, args, &jobs),
            "export" => export_to_csv(&context, args),
            "import" => import_from_csv(&mut context, args),
            _ => println!("invalid command!")
        }
    }
}