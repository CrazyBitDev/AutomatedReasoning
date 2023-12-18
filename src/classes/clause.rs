use std::collections::HashMap;

pub struct Clause {
    pub literals: Vec<i32>,
    pub is_satisfied: bool,
    pub is_always_satisfied: bool,
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            literals: Vec::new(),
            is_satisfied: false,
            is_always_satisfied: false,
        }
    }

    pub fn load_string(&mut self, literal_string: String) -> Result<(), ()> {

        if literal_string.trim().is_empty()
          || literal_string.trim().chars().next().unwrap().is_alphabetic()
          || literal_string.trim() == "0"
          || literal_string.trim() == "%" {
            return Err(());
        }

        self.literals = literal_string
            .split_whitespace()
            .filter_map(|literal| literal.parse().ok())
            .filter(|&literal| literal != 0)
            .collect();
        self.check_literals();

        Ok(())
    }

    pub fn load_vec(&mut self, literals: Vec<i32>) {
        self.literals = literals;
        self.check_literals();
    }

    pub fn add_literal(&mut self, literal: i32) {
        self.literals.push(literal);
        self.check_literals();
    }
    pub fn remove_literal(&mut self, literal: i32) {
        self.literals.retain(|&x| x != literal);
        self.check_literals();
    }

    fn check_literals(&mut self) {
        // if the clause contains a literal and its negation, the clause is always satisfied
        for literal in &self.literals {
            if self.literals.contains(&-literal) {
                self.is_always_satisfied = true;
                break;
            }
        }
    }

    pub fn check_satisfied(&mut self, instance: Vec<i32>, force_check: bool) -> bool {
        if self.is_always_satisfied || (self.is_satisfied && !force_check) {
            return true;
        }

        self.is_satisfied = false;
        for literal in &self.literals {
            // if the literal is present in the instance (No the absolute value!), the clause is satisfied
            if instance.contains(&literal) {
                self.is_satisfied = true;
                break;
            }
        }
        return self.is_satisfied;
    }

    pub fn print(&self) {
        let literals_str: Vec<String> = self.literals.iter().map(|&literal| literal.to_string()).collect();
        println!("{}", literals_str.join(" "));
    }

    fn print_with_chars(&self, literal_map: HashMap<u32, String>) {
        let literals_str: Vec<String> = self.literals
            .iter()
            .map(|&literal| literal_map.get(&(literal.abs() as u32)).unwrap().clone())
            .collect();
        println!("{}", literals_str.join(" "));
    }
}