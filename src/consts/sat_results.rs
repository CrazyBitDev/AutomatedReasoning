pub enum SatResultz {
    Satisfiable(Vec<i32>),
    Unsatisfiable,
    Unknown,
}

impl fmt::Display for SatResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SatResult::Unknown => write!(f, "Unknown"),
            SatResult::Satisfiable(_) => write!(f, "SAT"),
            SatResult::Unsatisfiable => write!(f, "UNSAT")
        }
    }
}