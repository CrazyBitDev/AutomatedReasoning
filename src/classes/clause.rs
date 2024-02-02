use std::{cmp, fmt, ops};
use std::slice::Iter;

use crate::consts::{sat::SAT, operators::OR};

use super::decision::Decision;
use super::model::Model;

#[derive(Clone)]
pub struct Clause {
    clause_id: usize,
    literals: Vec<isize>,
    satisfied: Option<usize>,
    is_always_satisfied: bool,

    two_watched_literals: (usize, usize),

    pub learned_clause_is_used_somewhere: bool
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            clause_id: 0,
            literals: Vec::new(),
            satisfied: None,
            is_always_satisfied: false,

            two_watched_literals: (0, 0),

            learned_clause_is_used_somewhere: false
        }
    }
    
    /// Set the clause id
    /// 
    /// # Arguments
    /// 
    /// * `id` - The id of the clause
    /// 
    pub fn set_id(&mut self, id: usize) {
        self.clause_id = id;
    }

    /// Get the clause id
    /// 
    /// # Returns
    /// 
    /// * `usize` - The id of the clause
    /// 
    pub fn get_id(&self) -> usize {
        self.clause_id
    }

    /// Load a string of literals into the clause
    /// 
    /// # Arguments
    /// 
    /// * `literal_string` - The string of literals
    /// 
    /// # Returns
    /// 
    /// * `Result<(), ()>` - Ok if the string was loaded successfully, Err if the string was invalid
    /// 
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

    /// Load a vector of literals into the clause
    /// 
    /// # Arguments
    /// 
    /// * `literals` - The vector of literals
    /// 
    pub fn load_vec(&mut self, literals: Vec<isize>) {
        self.literals = literals;
        self.check_literals();
    }

    /// Add a literal to the clause
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to add
    /// 
    pub fn add_literal(&mut self, literal: isize) {
        self.literals.push(literal);
        self.check_literals();
    }

    /// Remove a literal from the clause
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to remove
    /// 
    pub fn remove_literal(&mut self, literal: isize) {
        self.literals.retain(|&x| x != literal);
        self.check_literals();
    }

    /// Get the number of literals in the clause
    /// 
    /// # Returns
    /// 
    /// * `usize` - The number of literals in the clause
    /// 
    pub fn literals_len(&self) -> usize {
        self.literals.len()
    }

    /// Get the literal at a specific index
    /// 
    /// # Arguments
    /// 
    /// * `idx` - The index of the literal
    /// 
    /// # Returns
    /// 
    /// * `isize` - The literal at the index
    /// 
    pub fn get_literal(&self, idx: usize) -> isize {
        self.literals[idx]
    }

    /// Check if the clause contains a specific literal
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to check for
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the clause contains the literal, false otherwise
    /// 
    pub fn contains_literal(&self, literal: isize) -> bool {
        self.literals.contains(&literal)
    }

    /// Get an iterator over the literals in the clause
    /// 
    /// # Returns
    /// 
    /// * `Iter<isize>` - An iterator over the literals in the clause
    /// 
    pub fn iter_literals(&self) -> Iter<isize> {
        return self.literals.iter();
    }

    /// Check if the clause is a unit clause
    /// It is a unit clause if it contains only one literal, or the two watched literals are the same
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the clause is a unit clause, false otherwise
    /// 
    pub fn is_unit_clause(&self) -> bool {
        self.two_watched_literals.0 == self.two_watched_literals.1
    }
    
    /// Check if the clause is satisfied
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the clause is satisfied, false otherwise
    /// 
    pub fn is_satisfied(&self) -> bool {
        return self.satisfied.is_some() || self.is_always_satisfied;
    }

    /// Reset the satisfied flag of the clause
    /// 
    /// # Arguments
    /// 
    /// * `current_decision_level` - The current decision level
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the clause was reset, false otherwise
    /// 
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

    /// Get the watched literal index at a specific index
    /// 
    /// # Arguments
    /// 
    /// * `idx` - The index of the watched literal (0 or 1)
    /// 
    fn get_watched_literal_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            return self.two_watched_literals.0;
        } else {
            return self.two_watched_literals.1;
        }
    }

    /// Set the watched literal index at a specific index
    /// 
    /// # Arguments
    /// 
    /// * `idx` - The index of the watched literal (0 or 1)
    /// * `value` - The value to set the watched literal index to
    /// 
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
    
    /// Get the watched literals
    /// 
    /// # Returns
    /// 
    /// * `(isize, isize)` - The watched literals
    /// 
    pub fn get_watched_literals(&self) -> (isize, isize) {
        (self.literals[self.two_watched_literals.0], self.literals[self.two_watched_literals.1])
    }

    /// Check literals in the clause
    /// They are ordered by absolute value, duplicates are removed, and the clause is checked for always satisfied
    /// A clause is always satisfied if it contains a literal and its negation
    pub fn check_literals(&mut self) {
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
        } else {
            self.two_watched_literals = (0, 0)
        }
    }

    /// Check if a literal is satisfied by a model
    /// 
    /// # Arguments
    /// 
    /// * `model` - The model to check
    /// * `decision_level` - The decision level of the model
    /// 
    /// # Returns
    /// 
    /// * `SAT` - Satisfiable if the literal is satisfied, Unsatisfiable if the literal is not satisfied, Unknown if the literal is not assigned
    /// 
    pub fn is_satisfied_by_model(&mut self, model: &Model, decision_level: usize) -> SAT {

        if self.is_always_satisfied || self.satisfied.is_some() {
            return SAT::Satisfiable;
        }

        // two-watched literal propagation
        if self.is_unit_clause() {
            //println!("Testing unit clause: {}", self.get_literal(self.get_watched_literal_idx(0)));
            //let is_satisfied = self.check_literal_is_satisfied(self.get_watched_literal_idx(0), model);
            let is_satisfied = model.satisfies(self.get_literal(self.get_watched_literal_idx(0)));
            if is_satisfied == SAT::Satisfiable {
                self.satisfied = Some(decision_level);
            }
            return is_satisfied;
        } else {
            //print!("Testing clause: ");
            //self.print();
            'two_watched_literals_loop: loop {
                for i in 0..2 {
                    //match self.check_literal_is_satisfied(self.get_watched_literal_idx(i), model) {
                    match model.satisfies(self.get_literal(self.get_watched_literal_idx(i))) {
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

    /// Get the common literals between two clauses
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other clause
    /// 
    /// # Returns
    /// 
    /// * `Vec<isize>` - The common literals
    /// 
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

    /// Print the clause
    pub fn print(&self) {
        let literals_str: Vec<String> = self.literals.iter().map(|&literal| literal.to_string()).collect();
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