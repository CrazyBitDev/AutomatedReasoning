pub struct Decision {
    decided_literal: isize,
    
    propagated_literals: Vec<(isize, usize)>,
}

impl Decision {
    pub fn new(decided_literal: isize) -> Decision {
        Decision {
            decided_literal: decided_literal,
            
            propagated_literals: Vec::new(),
        }
    }
    
    /// Returns the decided literal
    /// 
    /// # Returns
    /// 
    /// * `isize` - The decided literal
    /// 
    pub fn get_decided_literal(&self) -> isize {
        self.decided_literal
    }
    
    /// Returns the propagated literals
    /// 
    /// # Returns
    /// 
    /// * `Vec<(isize, usize)>` - The propagated literals, where the first element is the literal and the second element is the index of the clause that propagated the literal
    /// 
    pub fn get_propagated_literals(&self) -> &Vec<(isize, usize)> {
        &self.propagated_literals
    }
    
    /// Adds a propagated literal
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to add
    /// * `clause_index` - The index of the clause that propagated the literal
    /// 
    pub fn add_propagated_literal(&mut self, literal: isize, clause_index: usize) {
        self.propagated_literals.push((literal, clause_index));
    }

    /// Clears the propagated literals
    pub fn clear_propagated_literals(&mut self) {
        self.propagated_literals.clear();
    }
}

//impl PartialEq
impl PartialEq for Decision {
    fn eq(&self, other: &Self) -> bool {
        if self.decided_literal == other.decided_literal {
            return true;
        }
        return self.propagated_literals
            .clone()
            .into_iter()
            .any(|(literal, _)| literal == other.decided_literal);
    }
}