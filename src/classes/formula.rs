use std::collections::HashSet;

use crate::{classes::clause::Clause, consts::sat::SAT};
use crate::files;

use super::model::Model;

pub struct Formula {
    clauses: Vec<Clause>,
    num_variables: usize,
    num_clauses: usize,

    current_clause_id: usize,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
            num_variables: 0,
            num_clauses: 0,

            current_clause_id: 0,
        }
    }

    /// Loads a file into the formula
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path of the file
    /// 
    /// # Returns
    /// 
    /// * `Result<(), std::io::Error>` - The result of the operation
    ///
    pub fn load_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = files::read_file(path);
        match data {
            Ok(contents) => {
                self.load_dimacs(contents);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Loads a string into the formula
    /// The string must be in DIMACS format
    /// 
    /// # Arguments
    /// 
    /// * `formula_string` - The string to load
    /// 
    pub fn load_dimacs(&mut self, formula_string: String) {
        let mut lines = formula_string.lines();
        //for each line
        for line in &mut lines {
            //if the line is a comment, skip it
            if line.starts_with('c') {
                continue;
            }
            //if the line is the problem line, parse the number of variables and clauses
            if line.starts_with("p") {
                let mut problem_line = line.split_whitespace();
                problem_line.next();
                problem_line.next();
                self.num_variables = problem_line.next().unwrap().parse().unwrap();
                self.num_clauses = problem_line.next().unwrap().parse().unwrap();
                continue;
            }
            //if the line is a clause, add it to the formula
            let mut clause = Clause::new();
            match clause.load_string(line.to_string()) {
                Ok(()) => {
                    self.current_clause_id += 1;
                    clause.set_id(self.current_clause_id);
                    self.clauses.push(clause)
                },
                Err(()) => continue,
            };
        }
    }

    /// Adds a clause to the formula
    /// 
    /// # Arguments
    /// 
    /// * `clause_string` - The clause to add
    /// 
    /// # Returns
    /// 
    /// * `Result<(), ()>` - The result of the operation, Ok(()) if successful, Err(()) if not
    /// 
    pub fn add_clause_by_string(&mut self, clause_string: String) -> Result<(), ()>{
        let mut clause = Clause::new();
        match clause.load_string(clause_string) {
            Ok(()) => {
                self.current_clause_id += 1;
                clause.set_id(self.current_clause_id);
                self.clauses.push(clause);
                return Ok(());
            },
            Err(_e) => return Err(()),
        };
    }

    /// Gets a clause by its id
    ///     
    /// # Arguments
    /// 
    /// * `id` - The id of the clause
    /// 
    /// # Returns
    /// 
    /// * `&Clause` - The clause
    /// 
    pub fn get_clause(&self, clause_idx: usize) -> &Clause {
        &self.clauses[clause_idx]
    }

    /// Gets a mutable reference to a clause by its id
    /// 
    /// # Arguments
    /// 
    /// * `id` - The id of the clause
    /// 
    /// # Returns
    /// 
    /// * `&mut Clause` - The clause
    /// 
    pub fn get_mut_clause(&mut self, clause_idx: usize) -> &mut Clause {
        self.clauses.get_mut(clause_idx).unwrap()
    }

    /// Returns the clauses
    /// 
    /// # Returns
    /// 
    /// * `&Vec<Clause>` - The clauses
    /// 
    pub fn get_clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }

    /// Returns a mutable reference to the clauses
    /// 
    /// # Returns
    /// 
    /// * `&mut Vec<Clause>` - The clauses
    /// 
    pub fn get_mut_clauses(&mut self) -> &mut Vec<Clause> {
        &mut self.clauses
    }

    /// Returns the number of variables
    /// 
    /// # Returns
    /// 
    /// * `usize` - The number of variables
    /// 
    pub fn get_num_variables(&self) -> usize {
        self.num_variables
    }

    /// Returns the number of clauses
    /// 
    /// # Returns
    /// 
    /// * `usize` - The number of clauses
    /// 
    pub fn get_num_clauses(&self) -> usize {
        self.num_clauses
    }

    /// Calculates the statistics of the formula
    pub fn calculate_stats(&mut self) {

        let mut variables = HashSet::new();

        for clause in &self.clauses {
            variables.extend(clause.iter_literals().map(|x| x.abs()));
        }

        self.num_variables = variables.len();
        self.num_clauses = self.clauses.len();
    }

    /// Returns the literals of the formula
    /// 
    /// # Returns
    /// 
    /// * `Vec<isize>` - The literals
    /// 
    pub fn get_all_watched_literals(&self) -> Vec<isize> {

        //create a map of literals where the value is the number of times the literal appears in unsatisfied clauses (the key is the absolute value of the literal)
        let mut literals: Vec<isize> = Vec::new();

        for clause in &self.clauses {
            if !clause.is_satisfied() {
                let literal_tuple = clause.get_watched_literals();
                literals.push(literal_tuple.0);
                if literal_tuple.0 != literal_tuple.1 {
                    literals.push(literal_tuple.1);
                }
            }
        }
        return literals;

    }

    /// Returns the next unit clause literal
    /// 
    /// # Arguments
    /// 
    /// * `model` - The model
    /// 
    /// # Returns
    /// 
    /// * `Option<(isize, usize)>` - The next unit clause literal, if there is one
    /// 
    pub fn get_next_unit_clause_literal(&mut self, model: &Model) -> Option<(isize, usize)> {
        for (clause_idx, clause) in self.clauses.iter_mut().enumerate() {
            if clause.is_unit_clause() && !model.has(clause.get_watched_literals().0) {
                return Some((clause.get_watched_literals().0, clause_idx));
            }
        }
        None
    }

    /// Checks if the formula is satisfied by the model
    /// 
    /// # Arguments
    /// 
    /// * `model` - The model
    /// * `decision_level` - The decision level
    /// 
    /// # Returns
    /// 
    /// * `SAT` - The result of the check
    /// * `usize` - The index of the unsatisfied clause, if there is one
    /// 
    pub fn is_satisfied(&mut self, model: &Model, decision_level: usize) -> (SAT, usize) {
        let mut satisfied = SAT::Satisfiable;
        for (idx, clause) in &mut self.clauses.iter_mut().enumerate() {
            match clause.is_satisfied_by_model(model, decision_level) {
                SAT::Satisfiable => continue,
                SAT::Unknown => satisfied = SAT::Unknown,
                SAT::Unsatisfiable => return (SAT::Unsatisfiable, idx), //conflict
            }
        }
        (satisfied, 0)
    }

    /// Prints the formula in CNF format
    pub fn print_cnf(&self) {
        for clause_idx in 0..self.clauses.len() {
            if clause_idx != 0 {
                print!("∧");
            }
            print!("(");
            for literal_idx in 0..self.clauses[clause_idx].literals_len() {
                if literal_idx != 0 {
                    print!("∨");
                }
                print!("{}", self.clauses[clause_idx].get_literal(literal_idx));
            }
            print!(")");
        }
        println!("\n");
    }

    /// Prints the formula in DIMACS format
    pub fn print_dimacs(&self) {
        println!("p cnf {} {}", self.num_variables, self.num_clauses);
        for clause in &self.clauses {
            println!("{} 0", clause.iter_literals().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
        println!("\n");
    }
}