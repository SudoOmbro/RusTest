use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::classes::*;
use crate::utils::get_gender_from_string;


const SEPARATOR: char = ';';


pub fn dump_context_to_csv(context: &Context, filename: &str) -> Result<String, String> {
    let result = File::create(filename);
    match result {
        Ok(mut file) => {
            for person in &context.people {
                let res = file.write_all(person.get_csv_line(SEPARATOR).as_bytes());
                match res {
                    Err(error) => { return Err(error.to_string()); },
                    _ => {}
                }
            }
            Ok(format!("Dumped people to {filename}"))
        },
        Err(error) => {
            Err(format!("Error: {}", error.to_string()))
        }
    }
}


fn parse_line(context: &mut Context, split_line: Vec<&str>) {
    let age: u32;
    let job: usize;
    match split_line[2].parse::<u32>() {
        Ok(input_age) => age = input_age,
        Err(_) => {
            println!("Wrong age format!");
            return ;
        }
    };
    match split_line[4][0..1].parse::<usize>() {
        Ok(input_job) => {
            if input_job < JOBS_NUM {
                job = input_job;
            }
            else {
                println!("Job out of range!");
                return ;
            }
        },
        Err(error) => {
            println!("Could not read job: {}", error.to_string());
            return ;
        }
    };
    context.add_person(
        split_line[0].to_string(), 
        split_line[1].to_string(), 
        age, 
        get_gender_from_string(split_line[3]), 
        job
    );
}


pub fn read_context_from_csv(context: &mut Context, filename: &str) -> Result<String, String> {
    let result = File::open(filename);
    match result {
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            loop {
                let res = buf_reader.read_line(&mut contents);
                let split_line: Vec<&str> = contents.split(SEPARATOR).collect();
                match res {
                    Ok(size) => {
                        if size == 0 {
                            return Ok(format!("File {filename} successfully imported"));
                        }
                        parse_line(context, split_line);
                        contents.clear();
                    },
                    Err(error) => { return Err(format!("An error has occurred while reading {filename}, error: {}", error.to_string())) }
                }
            }
        },
        Err(error) => {
            Err(format!("Could not read {filename}, error: {}", error.to_string()))
        }
    }
}