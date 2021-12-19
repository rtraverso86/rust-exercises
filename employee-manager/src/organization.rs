use std::collections::{HashMap, BTreeSet};

pub struct Organization {
    departments: HashMap<String, BTreeSet<String>>,
}

impl Organization {
    pub fn new() -> Organization {
        Organization {
            departments : HashMap::new(),
        }
    }

    pub fn add(&mut self, person: String, dept: String) {
       let dept_people = self.departments.entry(dept).or_insert(BTreeSet::new());
       dept_people.insert(person);
    }

    pub fn print_list(&self, dept: &str) {
        if let Some(dept_people) = self.departments.get(dept) {
            println!("Employees:");
            for p in dept_people {
                println!("  - {}", p);
            }
        } else {
            println!("|  Department <{}> not found", dept);
        }
    }
}

