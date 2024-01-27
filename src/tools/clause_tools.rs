use crate::classes::clause::Clause;
use crate::classes::model::Model;
use crate::consts::sat::SAT;

pub fn get_next_unit_clause_literal<'a>(clauses: &Vec<Clause>, model: &Model) -> Option<(isize, usize)> {
    for (clause_idx, clause) in clauses.iter().enumerate() {
        if clause.is_unit_clause() && !model.has(clause.get_watched_literals().0) && !clause.is_satisfied() {
            return Some((clause.get_watched_literals().0, clause_idx));
        }
    }
    None
}

pub fn clauses_are_satisfied(clauses: &mut Vec<Clause>, model: &Model, decision_level: usize, increase_idx: Option<usize>) -> (SAT, usize) {
    let mut satisfied = SAT::Satisfiable;
    let increase_idx = increase_idx.unwrap_or(0);
    for (idx, clause) in &mut clauses.iter_mut().enumerate() {
        match clause.is_satisfied_by_model(model, decision_level) {
            SAT::Satisfiable => continue,
            SAT::Unknown => satisfied = SAT::Unknown,
            SAT::Unsatisfiable => {
                return (SAT::Unsatisfiable, idx + increase_idx)
            }, //conflict
        }
    }
    (satisfied, 0)
}