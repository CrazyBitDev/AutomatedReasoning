use std::collections::HashMap;
use std::cmp;
use std::slice::Iter;

use crate::consts::sat::SAT;

pub struct Clause {
    literals: Vec<isize>,
    pub satisfied: bool,
    pub is_always_satisfied: bool,

    pub two_watched_literals: (usize, usize),
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            literals: Vec::new(),
            satisfied: false,
            is_always_satisfied: false,

            two_watched_literals: (0, 0),
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

    pub fn load_vec(&mut self, literals: Vec<isize>) {
        self.literals = literals;
        self.check_literals();
    }

    pub fn add_literal(&mut self, literal: isize) {
        self.literals.push(literal);
        self.check_literals();
    }
    pub fn remove_literal(&mut self, literal: isize) {
        self.literals.retain(|&x| x != literal);
        self.check_literals();
    }
    pub fn literals_len(&self) -> usize {
        self.literals.len()
    }
    pub fn get_literal(&self, idx: usize) -> isize {
        self.literals[idx]
    }
    pub fn iter_literals(&self) -> Iter<isize> {
        return self.literals.iter();
    }

    pub fn is_unit_clause(&self) -> bool {
        self.two_watched_literals.0 == self.two_watched_literals.1
    }
    
    pub fn is_satisfied(&self) -> bool {
        return self.satisfied;
    }
    pub fn reset_satisfied(&mut self) {
        self.satisfied = false;
    }

    fn get_watched_literal_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            return self.two_watched_literals.0;
        } else {
            return self.two_watched_literals.1;
        }
    }
    fn set_watched_literal_idx(&mut self, idx: usize, value: usize) {
        if idx == 0 {
            self.two_watched_literals.0 = value;
        } else {
            self.two_watched_literals.1 = value;
        }
    }
    
    pub fn get_watched_literals(&self) -> (isize, isize) {
        (self.literals[self.two_watched_literals.0], self.literals[self.two_watched_literals.1])
    }

    fn check_literals(&mut self) {
        // if the clause contains a literal and its negation, the clause is always satisfied
        for literal in &self.literals {
            if self.literals.contains(&-literal) {
                self.is_always_satisfied = true;
                break;
            }
        }
        if self.literals.len() > 1 {
            self.two_watched_literals = (0, 1)
        }
    }

    fn check_literal_is_satisfied(&mut self, literal_idx: usize, instance: &Vec<isize>) -> SAT {

        if instance.contains(&self.literals[literal_idx]) {
            return SAT::Satisfiable;
        } else if instance.contains(&(self.literals[literal_idx] * -1)) {
            return SAT::Unsatisfiable; // conflict
        }

        return SAT::Unknown;
    }

    pub fn is_satisfied_by_instance(&mut self, instance: &Vec<isize>) -> SAT {
        if self.is_always_satisfied || self.satisfied {
            return SAT::Satisfiable;
        }

        // two-watched literal propagation
        if self.is_unit_clause() {
            println!("Analyzing unit clause");
            let is_satisfied = self.check_literal_is_satisfied(self.get_watched_literal_idx(0), instance);
            self.satisfied = is_satisfied == SAT::Satisfiable;
            return is_satisfied;
        } else {
            println!("Analyzing clause");
            'two_watched_literals_loop: loop {
                for i in 0..2 {
                    match self.check_literal_is_satisfied(self.get_watched_literal_idx(i), instance) {
                        SAT::Satisfiable => {
                            self.satisfied = true;
                            return SAT::Satisfiable;
                        },
                        SAT::Unsatisfiable => {
                            let max = cmp::max(self.get_watched_literal_idx(0), self.get_watched_literal_idx(1));
                            if max == self.literals.len() - 1 {
                                if i == 0 {
                                    self.set_watched_literal_idx(i, max);
                                } else if self.get_watched_literal_idx(1) > self.get_watched_literal_idx(0) {
                                    return SAT::Unknown;
                                } else {
                                    return SAT::Unsatisfiable;
                                }
                            } else {
                                self.set_watched_literal_idx(i, max + 1);
                                continue 'two_watched_literals_loop;
                            }
                            
                        },
                        SAT::Unknown => (),
                    }
                }
                break;
            }
            return SAT::Unknown;
        }
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