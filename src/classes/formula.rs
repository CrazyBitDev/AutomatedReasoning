use crate::classes::clause::Clause;
use crate::files;

use std::collections::{HashMap, HashSet};

pub struct Formula {
    pub clauses: Vec<Clause>,
    pub num_variables: u32,
    pub num_clauses: u32,

    pub literal_map: HashMap<i32, String>,

    pub formula_string: Vec<String>,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
            num_variables: 0,
            num_clauses: 0,
            literal_map: HashMap::new(),

            formula_string: Vec::new()
        }
    }

    pub fn load_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let data = files::read_file(path);
        match data {
            Ok(contents) => {
                self.load_string(contents);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn load_string(&mut self, formula_string: String) {
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
                Ok(()) => self.clauses.push(clause),
                Err(()) => continue,
            };
        }
    }

    pub fn add_clause_by_string(&mut self, clause_string: String) -> Result<(), ()>{
        let mut clause = Clause::new();
        match clause.load_string(clause_string) {
            Ok(()) => {
                self.clauses.push(clause);
                return Ok(());
            },
            Err(e) => return Err(()),
        };
    }

    pub fn calculate_stats(&mut self) {

        let mut variables = HashSet::new();

        for clause in &self.clauses {
            for literal in &clause.literals {
                variables.insert(literal.abs());
            }
        }

        self.num_variables = variables.len() as u32;
        self.num_clauses = self.clauses.len() as u32;
    }

    pub fn print_cnf(&self) {
        for clause_idx in 0..self.clauses.len() {
            if clause_idx != 0 {
                print!("∧");
            }
            print!("(");
            for literal_idx in 0..self.clauses[clause_idx].literals.len() {
                if literal_idx != 0 {
                    print!("∨");
                }
                print!("{}", self.clauses[clause_idx].literals[literal_idx]);
            }
            print!(")");
        }
        println!("\n");
    }

    pub fn print_dimacs(&self) {
        println!("p cnf {} {}", self.num_variables, self.num_clauses);
        for clause in &self.clauses {
            println!("{} 0", clause.literals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
        println!("\n");
    }
}