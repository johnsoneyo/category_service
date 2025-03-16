use crate::data::Department;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use emojic::flat::{DEPARTMENT_STORE, GREEN_APPLE, GREEN_BOOK, INFORMATION, OFFICE_WORKER, PERSON_FEEDING_BABY, POLICE_OFFICER, STOP_SIGN, THUMBS_UP, WARNING, WAVING_HAND};

pub fn get_median(v: &mut Vec<i32>) -> Option<i32> {
    match v.capacity() < 3 {
        true => None,
        false => {
            v.sort_by(|a, b| a.cmp(b));
            let queue_capacity = v.len();

            let is_balanced_array = queue_capacity % 2 > 0;

            println!("sorted array {v:?}");
            match is_balanced_array {
                false => {
                    let last_index = queue_capacity / 2;
                    let median = v
                        .get(last_index)
                        .map(|n| n + v.get(last_index - 1).unwrap())
                        .map(|total| total / 2)
                        .unwrap_or(0);
                    Some(median)
                }
                true => Some(v[queue_capacity / 2]),
            }
        }
    }
}

pub fn get_mode(v: &mut Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();

    for num in v {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    let mode = map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(a, b)| *b)
        .unwrap();

    Some(mode)
}

pub fn add_employee() {
    println!("{} Welcome to add employee service", WAVING_HAND);

    let mut department_employees: HashMap<Department, Vec<String>> = HashMap::new();
    let mut department_employees_counter: HashMap<Department, u32> = HashMap::new();
    loop {
        println!("{} Please enter the employee name or Usage for instructions.", POLICE_OFFICER);
        let mut question = String::new();

        match io::stdin().read_line(&mut question) {
            Ok(n) => {

                match question.trim() {
                    "Usage" =>  {
                        println!("{} Usage: enter CTRL-C to stop {}", INFORMATION, STOP_SIGN);
                        println!("{} Usage: Print to show map", INFORMATION);
                    }
                    "Print" => {
                        println!("{} Departments to employees {department_employees:?}", THUMBS_UP)
                    }
                    _ => {
                        let trimmed_employee = question.trim_ascii();
                        println!("{} Please kindly enter the department of employee: {trimmed_employee}", DEPARTMENT_STORE);
                        let mut department = String::new();
                        io::stdin()
                            .read_line(&mut department)
                            .expect("Failed to read department");

                        let dept = Department::from_str(&department.trim_ascii())
                            .expect("couldn't parse department");

                         department_employees.entry(dept)
                             .or_insert_with(Vec::new)
                             .push(trimmed_employee.to_string());

                        // department_employees_counter.entry(dept)
                        //     .and_modify(|e| { *e += 1 })
                        //     .or_insert(1);

                    }
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
