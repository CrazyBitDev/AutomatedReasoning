use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum SAT {
    Satisfiable,
    Unsatisfiable,
    Unknown,
}

impl fmt::Display for SAT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Satisfiable => write!(f, "SAT"),
            Self::Unsatisfiable => write!(f, "UNSAT")
        }
    }
}

impl PartialEq for SAT {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Satisfiable, Self::Satisfiable) => true,
            (Self::Unsatisfiable, Self::Unsatisfiable) => true,
            (Self::Unknown, Self::Unknown) => true,
            _ => false,
        }
    }
}

impl std::ops::Add for SAT {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Satisfiable, Self::Satisfiable) => Self::Satisfiable,
            (_, Self::Unsatisfiable) => Self::Unsatisfiable,
            (Self::Unsatisfiable, _) => Self::Unsatisfiable,
            _ => Self::Unknown,
        }
    }
}

impl std::ops::AddAssign for SAT {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}