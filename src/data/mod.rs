use std::str::FromStr;

struct Employee {
    name: String,
    department: Department,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Department {
    Sales,
    Engineering,
}

impl FromStr for Department {
    type Err = ();

    fn from_str(input: &str) -> Result<Department, Self::Err> {
        match input {
            "eng" => Ok(Department::Engineering),
            "sal" => Ok(Department::Sales),
            _ => Err(()),
        }
    }
}
