use crate::classes::formula::Formula;

pub struct Solver {
    pub formula: Formula,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            formula: Formula::new(),
        }
    }
    
    pub fn reset(&mut self) {
        self.formula = Formula::new();
    }

    pub fn is_formula_loaded(&self) -> bool {
        return self.formula.num_clauses != 0;
    }

}