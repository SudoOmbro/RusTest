pub const JOBS_NUM: usize = 4;

#[derive(Copy, Clone, PartialEq)]
pub enum Gender {
    MALE,
    FEMALE
}


pub struct Context {
    pub last_id: u32,
    pub people: Vec<Person>
}

impl Context {

    pub fn add_person(&mut self, name: String, surname: String, age: u32, gender: Gender, job: usize) {
        self.people.push(Person {
            id: self.last_id, 
            name: name, 
            surname: surname, 
            age: age, 
            gender: gender, 
            job: job
        });
        self.last_id += 1;
    }

    pub fn remove_person_by_id(&mut self, id: u32) {
        self.people.retain(|person| person.id != id)
    }

    pub fn remove_person_by_name(&mut self, name: &str) {
        self.people.retain(|person| person.name != name)
    }

}

#[derive(Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub surname: String,
    pub age: u32,
    pub gender: Gender,
    pub job: usize
}

impl Person {

    pub fn new(id: u32, name: String, surname: String, age: u32, gender: Gender, job: usize) -> Person {
        Person {id, name, surname, age, gender, job}
    }

    pub fn get_formatted_info(&self, jobs: &[String; JOBS_NUM]) -> String {
        let gender_string: &str;
        match &self.gender {
            Gender::MALE => gender_string = "Male",
            Gender::FEMALE => gender_string = "Female"
        }
        format!("{} - {} {}, age: {}, {}, {}", self.id, self.name, self.surname, self.age, gender_string, jobs[self.job])
    }

    pub fn get_csv_line(&self, sep: char) -> String {
        let mut gender: char = 'M';
        if self.gender == Gender::FEMALE {
            gender = 'F';
        }
        format!("{}{sep}{}{sep}{}{sep}{gender}{sep}{}\n", self.name, self.surname, self.age, self.job)
    }

}