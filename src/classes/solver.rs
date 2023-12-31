use petgraph::graph::Graph;

use crate::classes::clause::Clause;
use crate::classes::formula::Formula;
use crate::consts::sat::SAT;

pub struct Solver {
    pub formula: Formula,

    learned_clauses: Vec<Clause>,

    instance: Vec<isize>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            formula: Formula::new(),

            learned_clauses: Vec::new(),

            instance: Vec::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.formula = Formula::new();
        self.reset_solve();
    }

    pub fn is_formula_loaded(&self) -> bool {
        return self.formula.num_clauses != 0;
    }


    pub fn reset_solve(&mut self) {
        self.instance = Vec::new();
        self.learned_clauses = Vec::new();
    }

    pub fn solve(&mut self) -> Result<SAT, ()> {

        if !self.is_formula_loaded() {
            return Err(());
        }

        'solve_loop: loop {

            'unit_clause_loop: loop {
                match self.formula.get_next_unit_clause_literal(&self.instance) {
                    Some(literal) => {
                        println!("Found unit clause: {}", literal);
                        println!("Instance: {:?}", self.instance);

                        if self.instance.contains(&(literal * -1)) {
                            println!("Conflict");
                            return Ok(SAT::Unsatisfiable);
                        }

                        self.instance.push(literal);

                        match self.formula.is_satisfied(&self.instance) {
                            SAT::Satisfiable => return Ok(SAT::Satisfiable),
                            SAT::Unsatisfiable => return Ok(SAT::Unsatisfiable), // conflict
                            SAT::Unknown => continue 'unit_clause_loop,
                        }

                    },
                    None => {
                        break 'unit_clause_loop;
                    }
                }
            }

            
            let decided_literal = self.decision_VSIDS();
            self.instance.push(decided_literal);

            match self.formula.is_satisfied(&self.instance) {
                SAT::Satisfiable => return Ok(SAT::Satisfiable),
                SAT::Unsatisfiable => {
                    self.instance.pop();
                    
                    //learning

                    //backjump
                },
                SAT::Unknown => continue 'solve_loop,
            }

        }

        Ok(SAT::Unknown)
    }

    pub fn print_instance(&self) {
        for literal in &self.instance {
            print!("{} ", literal);
        }
        println!("");
    }

    fn decision_VSIDS(&self) -> isize {
        let mut literals = self.formula.get_all_watched_literals();
        for clause in &self.learned_clauses {
            let literal_tuple = clause.get_watched_literals();
            literals.push(literal_tuple.0);
            if literal_tuple.0 != literal_tuple.1 {
                literals.push(literal_tuple.1);
            }
        }
        let mut literals_count = literals.clone();

        literals_count = literals_count.iter().map(|x| x.abs()).collect::<Vec<isize>>();
        literals_count.dedup();
        literals_count.sort();

        literals_count.sort_by(|a, b| literals.iter().filter(|&x| x.abs() == *b).count().cmp(&literals.iter().filter(|&x| x.abs() == *a).count()));

        let literal = literals_count[0];

        if literals.iter().filter(|&x| x.abs() == literal).count() / 2 >= literals.iter().filter(|&x| *x == literal * -1).count() {
            return literal;
        } else {
            return literal * -1;
        }
    }

}