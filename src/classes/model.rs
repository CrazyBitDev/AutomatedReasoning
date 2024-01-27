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

    pub fn resize(&mut self, size: usize) {
        self.model.resize(size, ModelValue::Unknown);
    }

    pub fn add(&mut self, literal: isize) {
        if literal > 0 {
            self.model[literal.abs() as usize - 1] = ModelValue::Positive;
        } else {
            self.model[literal.abs() as usize - 1] = ModelValue::Negative;
        }
    }

    pub fn remove(&mut self, literal: isize) {
        self.model[literal.abs() as usize - 1] = ModelValue::Unknown;
    }

    pub fn has(&self, literal: isize) -> bool {
        match self.model[literal.abs() as usize - 1] {
            ModelValue::Positive => literal > 0,
            ModelValue::Negative => literal < 0,
            ModelValue::Unknown => false,
        }
    }

    pub fn has_abs(&self, literal: usize) -> bool {
        match self.model[literal - 1] {
            ModelValue::Positive => true,
            ModelValue::Negative => true,
            ModelValue::Unknown => false,
        }
    }

    pub fn satisfies(&self, literal: isize) -> SAT {
        match (self.model[literal.abs() as usize - 1], literal > 0) {
            (ModelValue::Unknown, _) => SAT::Unknown,
            (ModelValue::Positive, true) => SAT::Satisfiable,
            (ModelValue::Negative, false) => SAT::Satisfiable,
            (_, _) => SAT::Unsatisfiable,
        }
    }


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