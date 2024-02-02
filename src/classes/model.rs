use crate::SAT;

#[derive(Clone, Copy, PartialEq)]
enum ModelValue {
    Positive,
    Negative,
    Unknown
}

pub struct Model {
    model: Vec<ModelValue>,
}

impl Model {
    pub fn new(size: Option<usize>) -> Model {
        let size = size.unwrap_or(0);
        Model {
            model: vec![ModelValue::Unknown; size],
        }
    }

    /// Resizes the model
    /// 
    /// # Arguments
    /// 
    /// * `size` - The new size
    /// 
    pub fn resize(&mut self, size: usize) {
        self.model.resize(size, ModelValue::Unknown);
    }

    /// Adds a literal to the model
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to add
    /// 
    pub fn add(&mut self, literal: isize) {
        if literal > 0 {
            self.model[literal.abs() as usize - 1] = ModelValue::Positive;
        } else {
            self.model[literal.abs() as usize - 1] = ModelValue::Negative;
        }
    }

    /// Removes a literal from the model
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to remove
    /// 
    pub fn remove(&mut self, literal: isize) {
        self.model[literal.abs() as usize - 1] = ModelValue::Unknown;
    }

    /// Checks if the model has a literal
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to check
    /// 
    /// # Returns
    /// 
    /// * `bool` - true if the model has the literal, false otherwise
    /// 
    pub fn has(&self, literal: isize) -> bool {
        match self.model[literal.abs() as usize - 1] {
            ModelValue::Positive => literal > 0,
            ModelValue::Negative => literal < 0,
            ModelValue::Unknown => false,
        }
    }

    /// Checks if the model has a literal, ignoring the sign
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to check
    /// 
    /// # Returns
    /// 
    /// * `bool` - true if the model has the literal, false otherwise
    /// 
    pub fn has_abs(&self, literal: usize) -> bool {
        match self.model[literal - 1] {
            ModelValue::Positive => true,
            ModelValue::Negative => true,
            ModelValue::Unknown => false,
        }
    }

    /// Checks if the model satisfies a literal
    /// 
    /// # Arguments
    /// 
    /// * `literal` - The literal to check
    /// 
    /// # Returns
    /// 
    /// * `SAT` - The result of the check
    /// 
    pub fn satisfies(&self, literal: isize) -> SAT {
        match (self.model[literal.abs() as usize - 1], literal > 0) {
            (ModelValue::Unknown, _) => SAT::Unknown,
            (ModelValue::Positive, true) => SAT::Satisfiable,
            (ModelValue::Negative, false) => SAT::Satisfiable,
            (_, _) => SAT::Unsatisfiable,
        }
    }

    /// Prints the model
    pub fn print(&self) {
        for (idx, value) in self.model.iter().enumerate() {
            match value {
                ModelValue::Positive => print!("{} ", idx + 1),
                ModelValue::Negative => print!("-{} ", idx + 1),
                ModelValue::Unknown => (),
            }
        }
        println!();
    }

}