use crate::classes::*;
use crate::random_utils::*;
use crate::csvutils::*;
use crate::utils::*;


pub fn add_person(context: &mut Context, args: Vec<&str>) {
    if args.len() == 1 {
        for arg in args {
            match arg {
                "r" => {
                    let name: String = randstr(randint(3, 15) as usize);
                    let surname: String = randstr(randint(3, 15) as usize);
                    let age: u32 = randint(3, 15) as u32;
                    let mut gender: Gender = Gender::MALE;
                    if randint(0, 2) == 0 {
                        gender = Gender::FEMALE
                    };
                    let job: usize = randint(0, 3) as usize;
                    context.add_person(name, surname, age, gender, job);
                    return ;
                },
                "d" => {
                    context.add_person(
                        "Jhon".to_string(), 
                        "Doe".to_string(), 
                        24, 
                        Gender::MALE, 
                        0
                    );
                    return;
                },
                _ => {}
            }
        }
    }
    else if args.len() == 5 {
        let age: u32;
        let job: usize;
        let gender: Gender;
        match args[2].parse::<u32>() {
            Ok(input_age) => age = input_age,
            Err(_) => {
                println!("Wrong age format!");
                return ;
            }
        }
        gender = get_gender_from_string(args[3]);
        match args[4].parse::<usize>() {
            Ok(input_job) => {
                if input_job < JOBS_NUM {
                    job = input_job;
                }
                else {
                    println!("Job out of range!");
                    return ;
                }
            },
            Err(_) => {
                println!("Wrong job format!");
                return ;
            }
        }
        context.add_person(
            args[0].to_string(), 
            args[1].to_string(), 
            age, 
            gender, 
            job
        );
        return ;
    }
    println!("wrong command format!")
}


pub fn remove_person(context: &mut Context, args: Vec<&str>) {
    for arg in args {
        match arg.parse::<u32>() {
            Ok(id) => context.remove_person_by_id(id),
            Err(_) => context.remove_person_by_name(arg)
        }
    }
}


pub fn rank_people(context: &Context, args: Vec<&str>, jobs: &[String; JOBS_NUM]) {
    let mut sorted_vector: Vec<Person> = context.people.clone();
    for arg in args {
        match arg {
            "id" => sorted_vector.sort_by(|a, b| a.id.cmp(&b.id)),
            "age" => sorted_vector.sort_by(|a, b| a.age.cmp(&b.age)),
            "name" => sorted_vector.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            "surname" => sorted_vector.sort_by(|a, b| a.surname.to_lowercase().cmp(&b.surname.to_lowercase())),
            "job" => sorted_vector.sort_by(|a, b| a.job.cmp(&b.job)),
            _ => {}
        }
    }
    for person in sorted_vector {
        println!("{}", person.get_formatted_info(&jobs));
    }
}


pub fn export_to_csv(context: &Context, args: Vec<&str>) {
    for arg in args {
        let result = dump_context_to_csv(context, arg);
        match result {
            Ok(message) => { println!("{message}") },
            Err(message) => { println!("{message}") }
        }
    }
}


pub fn import_from_csv(context: &mut Context, args: Vec<&str>) {
    for arg in args {
        let result = read_context_from_csv(context, arg);
        match result {
            Ok(message) => { println!("{message}") },
            Err(message) => { println!("{message}") }
        }
    }
}
