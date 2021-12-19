use std::collections::{BTreeMap, BTreeSet};

pub struct Organization {
    departments: BTreeMap<String, BTreeSet<String>>,
}

impl Organization {
    pub fn new() -> Organization {
        Organization {
            departments : BTreeMap::new(),
        }
    }

    pub fn add(&mut self, person: String, dept: String) {
       let dept_people = self.departments
           .entry(dept)
           .or_insert_with(BTreeSet::new);
       dept_people.insert(person);
    }

    pub fn print_list(&self, dept: &str) {
        if let Some(dept_people) = self.departments.get(dept) {
            println!("{} Employees:", dept);
            for p in dept_people {
                println!("  - {}", p);
            }
        } else {
            println!("|  Department <{}> not found", dept);
        }
    }

    pub fn print_all(&self) {
        for (dept, people) in &self.departments {
            println!("{} Employees:", dept);
            for p in people {
                println!("  - {}", p);
            }
        }
    }
}

