
pub struct Decision {
    decided_literal: isize,
    
    propagated_literals: Vec<isize>,
}

impl Decision {
    pub fn new(decided_literal: isize) -> Decision {
        Decision {
            decided_literal: decided_literal,
            
            propagated_literals: Vec::new(),
        }
    }
    
    pub fn get_decided_literal(&self) -> isize {
        self.decided_literal
    }
    
    pub fn get_propagated_literals(&self) -> &Vec<isize> {
        &self.propagated_literals
    }
    
    pub fn add_propagated_literal(&mut self, literal: isize) {
        self.propagated_literals.push(literal);
    }

    pub fn clear_propagated_literals(&mut self) {
        self.propagated_literals.clear();
    }
}

//impl PartialEq
impl PartialEq for Decision {
    fn eq(&self, other: &Self) -> bool {
        return self.decided_literal == other.decided_literal || self.propagated_literals.contains(&other.decided_literal);
    }
}