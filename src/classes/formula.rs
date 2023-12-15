use crate::classes::clause::Clause;
use crate::files;

use std::collections::HashMap;

pub struct Formula {
    pub clauses: Vec<Clause>,
    pub num_variables: u32,
    pub num_clauses: u32,

    pub literal_map: HashMap<i32, String>,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
            num_variables: 0,
            num_clauses: 0,
            literal_map: HashMap::new(),
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
}