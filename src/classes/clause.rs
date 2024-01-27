use std::collections::HashMap;
use std::{cmp, fmt, ops};
use std::slice::Iter;

use crate::consts::{sat::SAT, operators::OR};

use super::decision::Decision;
use super::instance::Instance;

#[derive(Clone)]
pub struct Clause {
    clause_id: usize,
    literals: Vec<isize>,
    satisfied: Option<usize>,
    is_always_satisfied: bool,

    two_watched_literals: (usize, usize),
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            clause_id: 0,
            literals: Vec::new(),
            satisfied: None,
            is_always_satisfied: false,

            two_watched_literals: (0, 0),
        }
    }
    
    pub fn set_id(&mut self, id: usize) {
        self.clause_id = id;
    }
    pub fn get_id(&self) -> usize {
        self.clause_id
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

    pub fn contains_literal(&self, literal: isize) -> bool {
        self.literals.contains(&literal)
    }

    pub fn iter_literals(&self) -> Iter<isize> {
        return self.literals.iter();
    }

    pub fn is_unit_clause(&self) -> bool {
        self.two_watched_literals.0 == self.two_watched_literals.1
    }
    
    pub fn is_satisfied(&self) -> bool {
        return self.satisfied.is_some() || self.is_always_satisfied;
    }

    pub fn reset_satisfied(&mut self, current_decision_level: usize) -> bool{

        if let Some(satisfied_level) = self.satisfied {
            if satisfied_level >= current_decision_level {
                self.satisfied = None;
                self.two_watched_literals.0 = 0;
                if self.literals.len() > 1 {
                    self.two_watched_literals.1 = 1;
                } else {
                    self.two_watched_literals.1 = 0;
                }
                return true;
            }
        } else {
            self.two_watched_literals.0 = 0;
            if self.literals.len() > 1 {
                self.two_watched_literals.1 = 1;
            } else {
                self.two_watched_literals.1 = 0;
            }
        }
        return false;
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
        if self.two_watched_literals.0 > self.two_watched_literals.1 {
            let temp = self.two_watched_literals.0;
            self.two_watched_literals.0 = self.two_watched_literals.1;
            self.two_watched_literals.1 = temp;
        }
    }
    
    pub fn get_watched_literals(&self) -> (isize, isize) {
        (self.literals[self.two_watched_literals.0], self.literals[self.two_watched_literals.1])
    }

    fn check_literals(&mut self) {
        // order by absolute value
        self.literals.sort_by(|a, b| a.abs().cmp(&b.abs()));
        // remove duplicates
        self.literals.dedup();
        // if the clause contains a literal and its negation, the clause is always satisfied
        self.is_always_satisfied = false;
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

    pub fn is_satisfied_by_instance(&mut self, instance: &Instance, decision_level: usize) -> SAT {
        if self.is_always_satisfied || self.satisfied.is_some() {
            return SAT::Satisfiable;
        }

        // two-watched literal propagation
        if self.is_unit_clause() {
            //println!("Testing unit clause: {}", self.get_literal(self.get_watched_literal_idx(0)));
            //let is_satisfied = self.check_literal_is_satisfied(self.get_watched_literal_idx(0), instance);
            let is_satisfied = instance.satisfies(self.get_literal(self.get_watched_literal_idx(0)));
            if is_satisfied == SAT::Satisfiable {
                self.satisfied = Some(decision_level);
            }
            return is_satisfied;
        } else {
            //print!("Testing clause: ");
            //self.print();
            'two_watched_literals_loop: loop {
                for i in 0..2 {
                    //match self.check_literal_is_satisfied(self.get_watched_literal_idx(i), instance) {
                    match instance.satisfies(self.get_literal(self.get_watched_literal_idx(i))) {
                        SAT::Satisfiable => {
                            self.satisfied = Some(decision_level);
                            return SAT::Satisfiable;
                        },
                        SAT::Unsatisfiable => {
                            if self.is_unit_clause() {
                                return SAT::Unsatisfiable;
                            }
                            let max = cmp::max(self.get_watched_literal_idx(0), self.get_watched_literal_idx(1));
                            if max == self.literals.len() - 1 {
                                if i == 0 {
                                    self.set_watched_literal_idx(i, max);
                                } else {
                                    self.set_watched_literal_idx(i, self.get_watched_literal_idx(0));
                                }
                            } else {
                                self.set_watched_literal_idx(i, max + 1);
                            }
                            continue 'two_watched_literals_loop;
                            
                        },
                        SAT::Unknown => (),
                    }
                }
                break;
            }
            return SAT::Unknown;
        }
    }


    pub fn is_assertion_clause(&self, decisions: &Vec<Decision>) -> bool {
        for literal in &self.literals {
            if !decisions.contains(&Decision::new(*literal)) && !decisions.contains(&Decision::new(-*literal)) {
                return false;
            }
        }
        return true;
    }

    pub fn get_common_literals(&self, other: &Clause) -> Vec<isize> {
        //find common literals
        let mut common_literals: Vec<isize> = Vec::new();
        for literal in &self.literals {
            if other.literals.contains(literal) {
                common_literals.push(*literal);
            }
        }
        return common_literals;
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

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literals_str: Vec<String> = self.literals.iter().map(|&literal| literal.to_string()).collect();
        write!(f, "{}", literals_str.join(&format!(" {} ", OR)))
    }
}

impl cmp::PartialEq for Clause {
    fn eq(&self, other: &Self) -> bool {
        self.literals == other.literals
    }
}

//implement the operator +, to merge two clauses
impl ops::Add for Clause {
    type Output = Clause;

    fn add(self, other: Clause) -> Clause {
        let mut new_clause = Clause::new();
        new_clause.literals = self.literals.clone();
        new_clause.literals.extend(other.literals.clone());
        new_clause.check_literals();
        return new_clause;
    }
}