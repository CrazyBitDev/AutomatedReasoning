use crate::consts::sat::SAT;
use crate::classes::{clause::Clause, model::Model};

/// Gets the next unit clause literal
/// 
/// # Arguments
/// 
/// * `clauses` - The clauses
/// * `model` - The model
/// 
/// # Returns
/// 
/// * `Option<(isize, usize)>` - The next unit clause literal, where the first element is the literal and the second element is the index of the clause.
/// If there is no unit clause, returns `None`
/// 
pub fn get_next_unit_clause_literal<'a>(clauses: &Vec<Clause>, model: &Model) -> Option<(isize, usize)> {
    for (clause_idx, clause) in clauses.iter().enumerate() {
        if clause.is_unit_clause() && !model.has(clause.get_watched_literals().0) && !clause.is_satisfied() {
            return Some((clause.get_watched_literals().0, clause_idx));
        }
    }
    None
}

/// Checks if the clauses are satisfied by the model
/// 
/// # Arguments
/// 
/// * `clauses` - The clauses
/// * `model` - The model
/// * `decision_level` - The decision level
/// * `increase_idx` - The amount to increase the index by (optional, default is 0). This is useful when the function is called for learned clauses,
/// as the index of the unsatisfied clause should be increased by the number of original clauses
/// 
/// # Returns
/// 
/// * `SAT` - The result of the check
/// * `usize` - The index of the unsatisfied clause, if there is one
/// 
pub fn clauses_are_satisfied(clauses: &mut Vec<Clause>, model: &Model, decision_level: usize, increase_idx: Option<usize>) -> (SAT, usize) {
    let mut satisfied = SAT::Satisfiable;
    let increase_idx = increase_idx.unwrap_or(0);
    for (idx, clause) in &mut clauses.iter_mut().enumerate() {
        match clause.is_satisfied_by_model(model, decision_level) {
            SAT::Satisfiable => continue,
            SAT::Unknown => satisfied = SAT::Unknown,
            SAT::Unsatisfiable => {
                return (SAT::Unsatisfiable, idx + increase_idx)
            },
        }
    }
    (satisfied, 0)
}