use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: i32,
}

pub struct StudentList {
    list: HashMap<String, Student>,
}

impl StudentList {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }

    fn add(&mut self, student: Student) {
        self.list.insert(student.name.to_string(), student);
    }

    fn view_all(&mut self) -> Vec<&Student> {
        self.list.values().collect()
    }

    fn edit(&mut self, name: &str, age: i32) -> bool {
        match self.list.get_mut(name) {
            Some(student) => {
                student.age = age;
                true
            }
            None => false,
        }
    }

    fn remove(&mut self, name: &str) -> bool {
        self.list.remove(name).is_some()
    }
}

mod manager {
    use crate::*;

    pub fn add_student(student_list: &mut StudentList) {
        println!("Please enter name of student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let age = match get_input_number() {
            Some(number) => number,
            None => return,
        };

        let student = Student { name, age };

        student_list.add(student);
    }

    pub fn view_all_student(student_list: &mut StudentList) {
        for student in student_list.view_all() {
            println!("{:?}", student)
        }
    }

    pub fn edit_student(student_list: &mut StudentList) {
        for student in student_list.view_all() {
            println!("{:?}", student)
        }
        println!("Enter name of student to edit:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let age = match get_input_number() {
            Some(number) => number,
            None => return,
        };
        if student_list.edit(&name, age) {
            println!("Success!");
        } else {
            println!("Failed!")
        }
    }

    pub fn remove_student(student_list: &mut StudentList) {
        for student in student_list.view_all() {
            println!("{:?}", student)
        }
        println!("Please enter name of student to remove");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if student_list.remove(&name) {
            println!("Success!")
        } else {
            println!("Failed!")
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again!")
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_input_number() -> Option<i32> {
    println!("Please enter age of student");
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

enum Manager {
    AddStudent,
    ViewAllStudent,
    UpdateStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("");
        println!("== MANAGER ==");
        println!("");
        println!("1. Add Student");
        println!("2. View All Student");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("");
        println!("Please enter your choice");
        println!("");
    }

    fn choose(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewAllStudent),
            "3" => Some(Manager::UpdateStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut student_list = StudentList::new();
    loop {
        Manager::show();
        let input = get_input().expect("No valid input, program has been interupt!");
        match Manager::choose(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut student_list),
            Some(Manager::ViewAllStudent) => manager::view_all_student(&mut student_list),
            Some(Manager::UpdateStudent) => manager::edit_student(&mut student_list),
            Some(Manager::DeleteStudent) => manager::remove_student(&mut student_list),
            None => break,
        }
    }
}
