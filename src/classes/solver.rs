use chrono::{DateTime, Utc};
use std::slice::Iter;
use std::vec;

use crate::classes::{clause::Clause, formula::Formula, decision::Decision, file::File};
use crate::tools::clause_tools;
use crate::consts::sat::SAT;


pub struct Solver {
    pub formula: Formula,

    learned_clauses: Vec<Clause>,

    instance: Vec<isize>,

    decision_level: usize,
    decisions: Vec<Decision>,

    vsids: Vec<(f32, f32)>,

    file: File
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            formula: Formula::new(),

            learned_clauses: Vec::new(),

            instance: Vec::new(),

            decision_level: 0,
            decisions: Vec::new(),

            vsids: Vec::new(),

            file: File::new(None)
        }
    }
    
    pub fn reset(&mut self) {
        self.formula = Formula::new();
        self.reset_solve();
    }
    
    pub fn reset_solve(&mut self) {
        self.instance = Vec::new();
        self.learned_clauses = Vec::new();
        self.decision_level = 0;
        self.decisions = Vec::new();
        self.vsids = Vec::new();

        self.formula.get_mut_clauses().iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });
    }


    pub fn is_formula_loaded(&self) -> bool {
        return self.formula.get_num_clauses() != 0;
    }

    pub fn add_learned_clause(&mut self, clause: Clause) -> usize{
        if !self.learned_clauses.contains(&clause) {
            self.learned_clauses.push(clause);
            return self.formula.get_num_clauses() + self.learned_clauses.len() - 1;
        }
        return 0;
    }

    fn get_clause(&self, idx: usize) -> &Clause {
        if idx < self.formula.get_num_clauses() {
            return self.formula.get_clause(idx);
        } else {
            return &self.learned_clauses[idx - self.formula.get_num_clauses()];
        }
    }
    fn get_mut_clause(&mut self, idx: usize) -> &mut Clause {
        if idx < self.formula.get_num_clauses() {
            return self.formula.get_mut_clause(idx);
        } else {
            return &mut self.learned_clauses[idx - self.formula.get_num_clauses()];
        }
    }

    fn num_clauses(&self) -> usize {
        return self.formula.get_num_clauses() + self.learned_clauses.len();
    }


    pub fn solve(&mut self) -> Result<SAT, ()> {

        if !self.is_formula_loaded() {
            return Err(());
        }

        self.decision_level = 0;
        self.decisions = vec![Decision::new(0)];

        self.vsids = vec![(0.0, 0.0); self.formula.get_num_variables()];

        self.file_init();

        'solve_loop: loop {

            'unit_clause_loop: loop {
                let mut instance_changed = false;
                'learned_clauses_loop: loop {
                    match clause_tools::get_next_unit_clause_literal(&mut self.learned_clauses, &self.instance) {
                        Some((literal, clause_idx)) => {
                            let clause_idx = clause_idx + self.formula.get_num_clauses();
                            self.instance.push(literal);
                            self.decisions[self.decision_level].add_propagated_literal(literal);
                            instance_changed = true;
                            let (satisfied, conflict_clause_idx) = self.check_if_satisfied();
                            match (satisfied) {
                                SAT::Satisfiable => {
                                    self.file_delete();
                                    return Ok(SAT::Satisfiable)
                                },
                                SAT::Unsatisfiable => {
                                    let solved = self.conflict_solver(literal, clause_idx, conflict_clause_idx);
                                    if !solved {
                                        self.file_close();
                                        return Ok(SAT::Unsatisfiable);
                                    }
                                    continue 'unit_clause_loop;
                                },
                                SAT::Unknown => continue,
                            }
                        },
                        None => {
                            break 'learned_clauses_loop;
                        }
                    }
                }
                'clauses_loop: loop {
                    match clause_tools::get_next_unit_clause_literal(self.formula.get_clauses(), &self.instance) {
                        Some((literal, clause_idx)) => {
                            self.instance.push(literal);
                            self.decisions[self.decision_level].add_propagated_literal(literal);
                            instance_changed = true;
                            let (satisfied, conflict_clause) = self.check_if_satisfied();
                            match (satisfied) {
                                SAT::Satisfiable => {
                                    self.file_delete();
                                    return Ok(SAT::Satisfiable)
                                },
                                SAT::Unsatisfiable => {
                                    let solved = self.conflict_solver(literal, clause_idx, conflict_clause);
                                    if !solved {
                                        self.file_close();
                                        return Ok(SAT::Unsatisfiable);
                                    }
                                    continue 'unit_clause_loop;
                                },
                                SAT::Unknown => continue,
                            }
                        },
                        None => {
                            break 'clauses_loop;
                        }
                    }
                }
                if !instance_changed {
                    break;
                }
            }

            let decided_literal = self.decision();
            self.instance.push(decided_literal);
            self.decision_level += 1;
            self.decisions.push(Decision::new(decided_literal));

            let (satisfied, _) = self.check_if_satisfied();
            match satisfied {
                SAT::Satisfiable => {
                    self.file_delete();
                    return Ok(SAT::Satisfiable)
                },
                _ => continue 'solve_loop,
            }

        }
    }

    fn check_if_satisfied(&mut self) -> (SAT, usize) {
        
        let (satisfied, conflict_clause) = clause_tools::clauses_are_satisfied(
            self.formula.get_mut_clauses(),
            &self.instance,
            self.decision_level,
            None
        );
        let (satisfied2, conflict_clause2) = clause_tools::clauses_are_satisfied(
            &mut self.learned_clauses,
            &self.instance,
            self.decision_level,
            Some(self.formula.get_num_clauses())
        );
        if satisfied == SAT::Unsatisfiable {
            return (satisfied, conflict_clause);
        }
        if satisfied2 == SAT::Unsatisfiable {
            return (satisfied2, conflict_clause2);
        }
        return (satisfied + satisfied2, 0)
    }

    pub fn print_instance(&self) {
        for literal in &self.instance {
            print!("{} ", literal);
        }
        println!("");
    }

    fn conflict_solver(&mut self, conflict_literal: isize, clause_idx: usize, conflict_clause_idx: usize) -> bool {

        let clause_idx = clause_idx;
        let conflict_clause_idx = conflict_clause_idx;

        let (_, new_clause) = self.explain_and_learn(conflict_literal, clause_idx, conflict_clause_idx);

        if new_clause.literals_len() == 0 {
            return false;
        }

        self.backjump();

        self.update_vsids(conflict_clause_idx);
        
        return true;

    }

    fn explain_and_learn(&mut self, conflict_literal: isize, clause_idx: usize, conflict_clause_idx: usize) -> (usize, Clause) {

        self.remove_latest_propagated_literals();

        let clause = self.get_clause(clause_idx);
        let conflict_clause = self.get_clause(conflict_clause_idx);
        
        let clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=clause_idx+1, clause=clause);
        let conflict_clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=conflict_clause_idx+1, clause=conflict_clause);

        //let common_literals = clause.get_common_literals(conflict_clause);
        let mut learned_clause = clause.clone() + conflict_clause.clone();
        learned_clause.remove_literal(conflict_literal);
        learned_clause.remove_literal(-conflict_literal);

        let learned_clause_formatted: String;
        let first_arrow_formatted: String;
        let second_arrow_formatted: String;
        let learned_clause_idx: usize;

        if learned_clause.literals_len() == 0 {
            learned_clause_idx = 0;

            learned_clause_formatted = "□".to_string();
            first_arrow_formatted = format!("{} -> □", clause_idx+1);
            second_arrow_formatted = format!("{} -> □", conflict_clause_idx+1);
        } else {
            learned_clause_idx = self.add_learned_clause(learned_clause.clone());

            learned_clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=learned_clause_idx+1, clause=learned_clause);
            first_arrow_formatted = format!("{} -> {}", clause_idx+1, learned_clause_idx + 1);
            second_arrow_formatted = format!("{} -> {}", conflict_clause_idx+1, learned_clause_idx + 1);

            //self.get_mut_clause(clause_idx).set_is_satisfied_somewhere(true);
            //self.get_mut_clause(conflict_clause_idx).set_is_satisfied_somewhere(true);
        }


        if clause_idx < self.formula.get_num_clauses() {
            self.file.writeln(&clause_formatted);
        }
        if conflict_clause_idx < self.formula.get_num_clauses()  {
            self.file.writeln(&conflict_clause_formatted);
        }
        self.file.writeln(&learned_clause_formatted);
        self.file.writeln(&first_arrow_formatted);
        self.file.writeln(&second_arrow_formatted);


        return (learned_clause_idx, learned_clause);

    }

    fn remove_latest_propagated_literals(&mut self) {
        self.decisions[self.decision_level].get_propagated_literals().iter().for_each(|&literal| {
            self.instance.retain(|&x| x != literal);
        });
        self.decisions[self.decision_level].clear_propagated_literals();
    }

    fn backjump(&mut self) {
        self.instance.retain(|&x| x != self.decisions[self.decision_level].get_decided_literal());

        if self.decision_level > 0 {
            self.decision_level -= 1;
        }
        self.decisions.pop();

        self.formula.get_mut_clauses().iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });
        self.learned_clauses.iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });

        if self.decisions.len() == 0 {
            self.decisions.push(Decision::new(0));
        }

        self.check_if_satisfied();
    }

    fn update_vsids(&mut self, conflict_clause_idx: usize) {
        let conflict_clause = self.get_clause(conflict_clause_idx);
        let mut vsids = self.vsids.clone();
        for literal in vsids.iter_mut() {
            *literal = (literal.0 / 2.0, literal.1 / 2.0);
            if literal.0 + literal.1 < f32::EPSILON {
                *literal = (0.0, 0.0);
            }
        }
        for literal in conflict_clause.iter_literals() {
            let vsids_literal = vsids.get_mut(literal.abs() as usize - 1);
            if literal > &0 {
                vsids_literal.unwrap().0 += 1.0;
            } else {
                vsids_literal.unwrap().1 += 1.0;
            }
        }
        self.vsids = vsids;
    }

    fn decision(&self) -> isize {
        //vsids
        let vsids = self.vsids.clone();
        let mut vsids: Vec<(usize, (f32, f32))> = vsids.iter().enumerate().map(|(idx, &value)| (idx, value)).collect();
        vsids.retain(|e| (e.1.0 + e.1.1) > 0.0 );
        vsids.sort_by(|a, b| (b.1.0+b.1.1).partial_cmp(&(a.1.0+a.1.1)).unwrap());
        
        for (idx, _) in vsids.iter() {
            if !self.instance.contains(&((idx + 1) as isize)) && !self.instance.contains(&((idx + 1) as isize * -1)) {
                let mut sign:isize = 1;
                if self.vsids[*idx].0 < self.vsids[*idx].1 {
                    sign = -1;
                }
                return (idx + 1) as isize * sign;
            }
        }



        let mut literals = self.formula.get_all_watched_literals();

        for clause in &self.learned_clauses {
            if !clause.is_satisfied() {
                let literal_tuple = clause.get_watched_literals();
                literals.push(literal_tuple.0);
                if literal_tuple.0 != literal_tuple.1 {
                    literals.push(literal_tuple.1);
                }
            }
        }
        
        let mut literals_count = literals.clone();

        literals_count = literals_count.iter().map(|x| x.abs()).collect::<Vec<isize>>();
        literals_count.dedup();
        literals_count.sort();

        literals_count.sort_by(|a, b| literals.iter().filter(|&x| x.abs() == *b).count().cmp(&literals.iter().filter(|&x| x.abs() == *a).count()));

        let literal = literals_count[0];

        let mut sign = 1;

        if literals.iter().filter(|&x| x.abs() == literal).count() / 2 < literals.iter().filter(|&x| *x == literal * -1).count() {
            sign = -1;
        }

        return literal * sign;
    }

    fn file_init(&mut self) {

        let current_time = Utc::now().format("%Y%m%d%H%M%S");

        self.file = File::new(Some(format!("proof_{}.dot", current_time)));
        self.file.create();
        self.file.writeln("digraph {");
    }

    fn file_close(&mut self) {
        self.file.writeln("}");
    }

    fn file_delete(&mut self) {
        self.file.delete();
    }

}