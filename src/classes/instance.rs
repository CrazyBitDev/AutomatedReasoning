use crate::SAT;

#[derive(Clone, Copy, PartialEq)]
enum InstanceValue {
    Positive,
    Negative,
    Unknown
}

pub struct Instance {
    instance: Vec<InstanceValue>,
}

impl Instance {
    pub fn new(size: Option<usize>) -> Instance {
        let size = size.unwrap_or(0);
        Instance {
            instance: vec![InstanceValue::Unknown; size],
        }
    }

    pub fn resize(&mut self, size: usize) {
        self.instance.resize(size, InstanceValue::Unknown);
    }

    pub fn add(&mut self, literal: isize) {
        if literal > 0 {
            self.instance[literal.abs() as usize - 1] = InstanceValue::Positive;
        } else {
            self.instance[literal.abs() as usize - 1] = InstanceValue::Negative;
        }
    }

    pub fn remove(&mut self, literal: isize) {
        self.instance[literal.abs() as usize - 1] = InstanceValue::Unknown;
    }

    pub fn has(&self, literal: isize) -> bool {
        match self.instance[literal.abs() as usize - 1] {
            InstanceValue::Positive => literal > 0,
            InstanceValue::Negative => literal < 0,
            InstanceValue::Unknown => false,
        }
    }

    pub fn has_abs(&self, literal: usize) -> bool {
        match self.instance[literal - 1] {
            InstanceValue::Positive => true,
            InstanceValue::Negative => true,
            InstanceValue::Unknown => false,
        }
    }

    pub fn satisfies(&self, literal: isize) -> SAT {
        match (self.instance[literal.abs() as usize - 1], literal > 0) {
            (InstanceValue::Unknown, _) => SAT::Unknown,
            (InstanceValue::Positive, true) => SAT::Satisfiable,
            (InstanceValue::Negative, false) => SAT::Satisfiable,
            (_, _) => SAT::Unsatisfiable,
        }
    }


    pub fn print(&self) {
        for (idx, value) in self.instance.iter().enumerate() {
            match value {
                InstanceValue::Positive => print!("{} ", idx + 1),
                InstanceValue::Negative => print!("-{} ", idx + 1),
                InstanceValue::Unknown => (),
            }
        }
        println!();
    }

}