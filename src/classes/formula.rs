use std::collections::{HashMap, HashSet};

use crate::{classes::clause::Clause, consts::sat::SAT};
use crate::files;

use super::model::Model;

pub struct Formula {
    pub clauses: Vec<Clause>,
    pub num_variables: usize,
    pub num_clauses: usize,

    pub literal_map: HashMap<usize, String>,

    pub formula_string: Vec<String>,

    current_clause_id: usize,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
            num_variables: 0,
            num_clauses: 0,
            literal_map: HashMap::new(),

            formula_string: Vec::new(),

            current_clause_id: 0,
        }
    }

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

    pub fn get_clause(&self, clause_idx: usize) -> &Clause {
        &self.clauses[clause_idx]
    }
    pub fn get_mut_clause(&mut self, clause_idx: usize) -> &mut Clause {
        self.clauses.get_mut(clause_idx).unwrap()
    }

    pub fn get_clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }
    pub fn get_mut_clauses(&mut self) -> &mut Vec<Clause> {
        &mut self.clauses
    }

    pub fn get_num_variables(&self) -> usize {
        self.num_variables
    }
    pub fn get_num_clauses(&self) -> usize {
        self.num_clauses
    }

    pub fn calculate_stats(&mut self) {

        let mut variables = HashSet::new();

        for clause in &self.clauses {
            variables.extend(clause.iter_literals().map(|x| x.abs()));
        }

        self.num_variables = variables.len();
        self.num_clauses = self.clauses.len();
    }

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




    pub fn get_next_unit_clause_literal(&mut self, model: &Model) -> Option<(isize, usize)> {
        for (clause_idx, clause) in self.clauses.iter_mut().enumerate() {
            if clause.is_unit_clause() && !model.has(clause.get_watched_literals().0) {
                return Some((clause.get_watched_literals().0, clause_idx));
            }
        }
        None
    }

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

    pub fn print_dimacs(&self) {
        println!("p cnf {} {}", self.num_variables, self.num_clauses);
        for clause in &self.clauses {
            println!("{} 0", clause.iter_literals().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
        println!("\n");
    }
}