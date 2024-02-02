use std::vec;
use chrono::Utc;

use crate::tools::clause_tools;
use crate::consts::{sat::SAT, operators};
use crate::classes::{clause::Clause, formula::Formula, decision::Decision, file::File, model::Model, stats::Stats};


pub struct Solver {
    pub formula: Formula,

    learned_clauses: Vec<Clause>,
    current_learned_clause_id: usize,
    max_learned_clauses: usize,

    model: Model,

    decision_level: usize,
    decisions: Vec<Decision>,
    vsids: Vec<(f32, f32)>,

    stats: Stats,

    print_dot_proof: bool,
    file_dot: File,

    print_txt_proof: bool,
    file_txt: File,

    print_tex_proof: bool,
    file_tex: File
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            formula: Formula::new(),

            learned_clauses: Vec::new(),
            current_learned_clause_id: 0,
            max_learned_clauses: 0,

            model: Model::new(None),

            decision_level: 0,
            decisions: Vec::new(),
            vsids: Vec::new(),

            stats: Stats::new(),

            print_dot_proof: false,
            file_dot: File::new(None),

            print_txt_proof: false,
            file_txt: File::new(None),

            print_tex_proof: false,
            file_tex: File::new(None)
        }
    }
    
    /// Reset the solver to its initial state.
    pub fn reset(&mut self) {
        self.formula = Formula::new();
        self.current_learned_clause_id = 0;
        self.reset_solve();
    }
    
    /// Reset the solver to its initial state, but keep the formula.
    pub fn reset_solve(&mut self) {
        self.model = Model::new(None);
        self.learned_clauses = Vec::new();
        self.decision_level = 0;
        self.decisions = Vec::new();
        self.vsids = Vec::new();
        self.stats = Stats::new();
        self.max_learned_clauses = 0;

        self.formula.get_mut_clauses().iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });
    }

    /// Check if the formula is loaded.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the formula is loaded, false otherwise.
    /// 
    pub fn is_formula_loaded(&self) -> bool {
        return self.formula.get_num_clauses() != 0;
    }

    /// Add a clause to the formula as a learned clause.
    /// 
    /// # Arguments
    /// 
    /// * `clause` - The clause to add to the formula.
    /// 
    /// # Returns
    /// 
    /// * `usize` - The index of the learned clause.
    /// 
    pub fn add_learned_clause(&mut self, clause: Clause) -> usize{
        if !self.learned_clauses.contains(&clause) {
            self.learned_clauses.push(clause);
            return self.formula.get_num_clauses() + self.learned_clauses.len() - 1;
        }
        return 0;
    }

    /// Returns a reference to the clause at the given index.
    /// If the index is out of bounds, returns a reference from the learned clauses.
    /// 
    /// # Arguments
    /// 
    /// * `idx` - The index of the clause.
    /// 
    /// # Returns
    /// 
    /// * `&Clause` - The reference to the clause.
    /// 
    fn get_clause(&self, idx: usize) -> &Clause {
        if idx < self.formula.get_num_clauses() {
            return self.formula.get_clause(idx);
        } else {
            return &self.learned_clauses[idx - self.formula.get_num_clauses()];
        }
    }

    /// Returns a mutable reference to the clause at the given index.
    /// If the index is out of bounds, returns a mutable reference from the learned clauses.
    /// 
    /// # Arguments
    /// 
    /// * `idx` - The index of the clause.
    /// 
    /// # Returns
    /// 
    /// * `&mut Clause` - The mutable reference to the clause.
    /// 
    fn get_mut_clause(&mut self, idx: usize) -> &mut Clause {
        if idx < self.formula.get_num_clauses() {
            return self.formula.get_mut_clause(idx);
        } else {
            return &mut self.learned_clauses[idx - self.formula.get_num_clauses()];
        }
    }

    /// Check if the dot proof is enabled.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the dot proof is enabled, false otherwise.
    /// 
    pub fn is_dot_proof_enabled(&self) -> bool {
        return self.print_dot_proof;
    }

    /// Check if the txt proof is enabled.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the txt proof is enabled, false otherwise.
    /// 
    pub fn is_txt_proof_enabled(&self) -> bool {
        return self.print_txt_proof;
    }

    /// Check if the tex proof is enabled.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the tex proof is enabled, false otherwise.
    /// 
    pub fn is_tex_proof_enabled(&self) -> bool {
        return self.print_tex_proof;
    }

    /// Set the dot proof to be enabled or disabled.
    /// 
    /// # Arguments
    /// 
    /// * `enable` - The value to set the dot proof to.
    /// 
    pub fn set_dot_proof_enabled(&mut self, enable: bool) {
        self.print_dot_proof = enable;
    }

    /// Set the txt proof to be enabled or disabled.
    /// 
    /// # Arguments
    /// 
    /// * `enable` - The value to set the txt proof to.
    /// 
    pub fn set_txt_proof_enabled(&mut self, enable: bool) {
        self.print_txt_proof = enable;
    }

    /// Set the tex proof to be enabled or disabled.
    /// 
    /// # Arguments
    /// 
    /// * `enable` - The value to set the tex proof to.
    /// 
    pub fn set_tex_proof_enabled(&mut self, enable: bool) {
        self.print_tex_proof = enable;
    }

    /// Main function to solve the formula.
    /// Returns the result of the formula if it is satisfiable, unsatisfiable or unknown.
    /// Returns an error if the formula is not loaded.
    /// 
    pub fn solve(&mut self) -> Result<SAT, ()> {

        if !self.is_formula_loaded() {
            return Err(());
        }

        self.decision_level = 0;
        self.decisions = vec![Decision::new(0)];
        self.model.resize(self.formula.get_num_variables());
        self.vsids = vec![(0.0, 0.0); self.formula.get_num_variables()];
        self.current_learned_clause_id = self.formula.get_num_clauses();
        self.max_learned_clauses = self.formula.get_num_clauses();

        self.file_init();

        // Main loop to solve the formula.
        'solve_loop: loop {

            // Unit clause loop.
            'unit_clause_loop: loop {
                let mut model_changed = false;

                // Learned clauses loop.
                'learned_clauses_loop: loop {
                    match clause_tools::get_next_unit_clause_literal(&mut self.learned_clauses, &self.model) {
                        Some((literal, clause_idx)) => {
                            let clause_idx = clause_idx + self.formula.get_num_clauses();
                            self.model.add(literal);
                            self.decisions[self.decision_level].add_propagated_literal(literal, clause_idx);
                            self.tex_print_model("Propagation", None);
                            model_changed = true;
                            let (satisfied, conflict_clause_idx) = self.check_if_satisfied();
                            match satisfied {
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

                // Clauses loop.
                'clauses_loop: loop {
                    match clause_tools::get_next_unit_clause_literal(self.formula.get_clauses(), &self.model) {
                        Some((literal, clause_idx)) => {
                            self.model.add(literal);
                            self.decisions[self.decision_level].add_propagated_literal(literal, clause_idx);
                            self.tex_print_model("Propagation", None);
                            model_changed = true;
                            let (satisfied, conflict_clause_idx) = self.check_if_satisfied();
                            match satisfied {
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
                            break 'clauses_loop;
                        }
                    }
                }
                if !model_changed {
                    break;
                }
            }

            let decided_literal = self.decision();
            self.model.add(decided_literal);
            self.decision_level += 1;
            self.decisions.push(Decision::new(decided_literal));

            self.tex_print_model("Decision", None);

            self.stats.update();

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

    /// Check if the formula is satisfiable.
    /// 
    /// # Returns
    /// 
    /// * `SAT` - The result of the formula, it is satisfiable, unsatisfiable or unknown.
    /// * `usize` - The index of the conflict clause if the formula is unsatisfiable.
    fn check_if_satisfied(&mut self) -> (SAT, usize) {
        
        let (satisfied, conflict_clause) = clause_tools::clauses_are_satisfied(
            self.formula.get_mut_clauses(),
            &self.model,
            self.decision_level,
            None
        );
        let (satisfied2, conflict_clause2) = clause_tools::clauses_are_satisfied(
            &mut self.learned_clauses,
            &self.model,
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

    /// Print the model of the formula.
    pub fn print_model(&self) {
        self.model.print();
    }

    /// Conflict solver function.
    /// 
    /// # Arguments
    /// 
    /// * `conflict_literal` - The literal that caused the conflict.
    /// * `clause_idx` - The index of the clause that caused the conflict.
    /// * `conflict_clause_idx` - The index of the conflict clause.
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the conflict was solved, false if the formula is unsatisfiable.
    /// 
    fn conflict_solver(&mut self, conflict_literal: isize, clause_idx: usize, conflict_clause_idx: usize) -> bool {

        self.update_vsids(conflict_clause_idx);
        self.tex_print_model("Conflict", Some(format!("{}: {}", self.get_clause(conflict_clause_idx).get_id(), self.get_clause(conflict_clause_idx))));

        let new_clause = self.explain(conflict_literal, clause_idx, conflict_clause_idx);

        if new_clause.literals_len() == 0 {
            return false;
        }

        self.add_learned_clause(new_clause.clone());
    
        self.tex_print_model("Learn", Some(format!("{}: {}", new_clause.get_id(), new_clause)));

        self.stats.increase_learned();

        self.stats.update();

        if self.learned_clauses.len() > self.max_learned_clauses {
            self.forget();
        }

        self.backjump();

        
        return true;

    }

    /// Explain function.
    /// 
    /// # Arguments
    /// 
    /// * `conflict_literal` - The literal that caused the conflict.
    /// * `clause_idx` - The index of the clause that caused the conflict.
    /// * `conflict_clause_idx` - The index of the conflict clause.
    /// 
    /// # Returns
    /// 
    /// * `Clause` - The learned clause.
    /// 
    fn explain(&mut self, conflict_literal: isize, clause_idx: usize, conflict_clause_idx: usize) -> Clause {

        self.remove_latest_propagated_literals();

        let clause = self.get_clause(clause_idx);
        let conflict_clause = self.get_clause(conflict_clause_idx);
        
        let clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=clause.get_id(), clause=clause);
        let conflict_clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=conflict_clause.get_id(), clause=conflict_clause);

        //let common_literals = clause.get_common_literals(conflict_clause);
        let mut learned_clause = clause.clone() + conflict_clause.clone();
        learned_clause.remove_literal(conflict_literal);
        learned_clause.remove_literal(-conflict_literal);
        learned_clause.check_literals();
        let current_learned_clause_id = self.current_learned_clause_id + 1;
        learned_clause.set_id(current_learned_clause_id);

        let learned_clause_formatted: String;
        let first_arrow_formatted: String;
        let second_arrow_formatted: String;
        
        let mut txt_formatted = format!("({}) {} - ({}) {} => ", clause.get_id(), clause, conflict_clause.get_id(), conflict_clause);

        if learned_clause.literals_len() == 0 {

            learned_clause_formatted = "□".to_string();
            first_arrow_formatted = format!("{} -> □", clause.get_id());
            second_arrow_formatted = format!("{} -> □", conflict_clause.get_id());
            
            txt_formatted.push_str("□");

            if self.print_tex_proof {
                self.tex_print_arrow("Fail");
                self.file_tex.writeln("$\\square$");
            }


        } else {
            learned_clause_formatted = format!("{id} [label=<<FONT POINT-SIZE='8.0'>({id})  </FONT>{clause}>]", id=learned_clause.get_id(), clause=learned_clause);
            first_arrow_formatted = format!("{} -> {}", clause.get_id(), learned_clause.get_id());
            second_arrow_formatted = format!("{} -> {}", conflict_clause.get_id(), learned_clause.get_id());
            
            txt_formatted.push_str(format!("({}) {}", learned_clause.get_id(), learned_clause).as_str());

            self.tex_print_model("Explain", Some(format!("{}: {}", learned_clause.get_id(), learned_clause)));

        }


        if self.print_dot_proof {
            if clause_idx < self.formula.get_num_clauses() {
                self.file_dot.writeln(&clause_formatted);
            }
            if conflict_clause_idx < self.formula.get_num_clauses()  {
                self.file_dot.writeln(&conflict_clause_formatted);
            }
            self.file_dot.writeln(&learned_clause_formatted);
            self.file_dot.writeln(&first_arrow_formatted);
            self.file_dot.writeln(&second_arrow_formatted);
        }
        if self.print_txt_proof {
            self.file_txt.writeln(&txt_formatted);
        }

        self.get_mut_clause(clause_idx).learned_clause_is_used_somewhere = true;
        self.get_mut_clause(conflict_clause_idx).learned_clause_is_used_somewhere = true;

        self.current_learned_clause_id = current_learned_clause_id;

        return learned_clause;

    }

    /// Remove latest propagated literals function.
    fn remove_latest_propagated_literals(&mut self) {
        self.decisions[self.decision_level].get_propagated_literals().iter().for_each(|&literal| {
            self.model.remove(literal.0);
        });
        self.decisions[self.decision_level].clear_propagated_literals();
        self.formula.get_mut_clauses().iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });
        self.learned_clauses.iter_mut().for_each(|clause| {
            clause.reset_satisfied(self.decision_level);
        });
    }

    /// Backjump function.
    fn backjump(&mut self) {

        self.remove_latest_propagated_literals();
        if self.decision_level > 0 {
            self.model.remove(self.decisions[self.decision_level].get_decided_literal());
            self.decision_level -= 1;
        }
        self.decisions.pop();

        if self.decisions.len() == 0 {
            self.decisions.push(Decision::new(0));
        }
        
        self.tex_print_model("Backjump", None);

        self.check_if_satisfied();

    }

    /// Update vsids function.
    /// 
    /// # Arguments
    /// 
    /// * `conflict_clause_idx` - The index of the conflict clause.
    /// 
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

    /// Decision function.
    /// It decides which literal to assign next. It uses the vsids heuristic.
    /// If the vsids heuristic is not enough, it uses the watched literals, counting the number of occurrences of each literal.
    /// 
    /// # Returns
    /// 
    /// * `isize` - The literal to decide.
    /// 
    fn decision(&self) -> isize {
        let vsids = self.vsids.clone();
        let mut vsids: Vec<(usize, (f32, f32))> = vsids.iter().enumerate().map(|(idx, &value)| (idx, value)).collect();
        vsids.retain(|e| (e.1.0 + e.1.1) > 0.0 );
        vsids.sort_by(|a, b| (b.1.0+b.1.1).partial_cmp(&(a.1.0+a.1.1)).unwrap());
        
        for (idx, _) in vsids.iter() {
            if !self.model.has_abs(idx + 1) {
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

    /// Forget function.
    /// It forgets half of the learned clauses.
    fn forget(&mut self) {

        //clone learned clauses and associate them with their id
        let mut learned_clauses: Vec<(usize, Clause)> = self.learned_clauses.clone()
                .iter_mut().enumerate().map(|(idx, clause)| (idx, clause.clone())).collect();

        let mut avg_clause_len = 0;
        for clause in learned_clauses.iter() {
            avg_clause_len += clause.1.literals_len();
        }
        avg_clause_len /= learned_clauses.len();

        let clauses_to_forget_target = self.learned_clauses.len() / 2;
        let mut clauses_forgotten = 0;
        let mut append = String::new();

        for clause in learned_clauses.iter_mut() {
           // if clause.1.literals_len() > 1 && clause.0 < clauses_to_forget_target && clause.1.learned_clause_is_used_somewhere {
            if clause.0 < clauses_to_forget_target && clause.1.literals_len() > avg_clause_len {
                self.learned_clauses.retain(|x| x.get_id() != clause.1.get_id());
                if clauses_forgotten > 0 {
                    append.push_str(", ");
                }
                append.push_str(&format!("{}", clause.1.get_id()));
                clauses_forgotten += 1;
            }
        }

        self.stats.increase_forgotten(clauses_forgotten);
        self.max_learned_clauses = (self.max_learned_clauses as f32 * 1.5).round() as usize;

        self.tex_print_model("Forget", Some(format!("{}", append)));

    }

    /// Print the arrow in the tex proof.
    /// 
    /// # Arguments
    /// 
    /// * `arrow_str` - The string to print below the arrow.
    /// 
    fn tex_print_arrow(&mut self, arrow_str: &str) {
        if self.print_tex_proof {
            self.file_tex.write(format!("\n\n$\\xRightarrow[\\text{{{}}}]{{}}$ ", arrow_str).as_str());
        }
    }

    /// Print the model in the tex proof.
    /// 
    /// # Arguments
    /// 
    /// * `arrow_str` - The string to print below the arrow.
    /// * `append` - The string to append to the model.
    ///
    fn tex_print_model(&mut self, arrow_str: &str, append: Option<String>) {

        if self.print_tex_proof {
        
            let mut model_string: String = String::new();
            let mut i = 0;
            for decision in self.decisions.iter() {
                if decision.get_decided_literal() != 0 {
                    model_string.push_str(&format!("{}{{^d}} ", decision.get_decided_literal()));
                    i += 1;
                }
                for propagated_literal in decision.get_propagated_literals() {
                    model_string.push_str(&format!("{}{{_{{{}}}}}", propagated_literal.0, propagated_literal.1+1));
                    i += 1;
                }
                if i > 40 {
                    model_string.push_str("\n");
                    i = 0;
                }
            }
            if model_string.len() == 0 {
                model_string.push_str("\\emptyset");
            }
            model_string.push_str("\n||F");
            let learned_clauses_len = self.learned_clauses.len();
            if learned_clauses_len > 0 {
                model_string.push_str("\\cup\\{");
                let mut i = 0;
                for (idx, clause) in self.learned_clauses.iter().enumerate() {
                    model_string.push_str(&format!("{}", clause.get_id()));
                    if idx < learned_clauses_len - 1 {
                        model_string.push_str(", ");
                    }
                    i += 1;
                    if i > 40 {
                        model_string.push_str("\n");
                        i = 0;
                    }
                }
                model_string.push_str("\\}");
            }

            let append_str = match append {
                Some(append) => format!("||{}", append),
                None => "".to_string()
            };

            self.tex_print_arrow(arrow_str);
            self.file_tex.write(
                &format!("\\overflow{{{}\n{}}} ", model_string, append_str).as_str()
                    .replace(operators::AND, "\\land")
                    .replace(operators::OR, "\\lor")
                    .replace("-", "\\neg")
            );

        }
    }

    /// Initialize the proof files.
    fn file_init(&mut self) {

        let current_time = Utc::now();
        let current_time_str = current_time.format("%Y%m%d%H%M%S");

        if self.print_dot_proof {
            self.file_dot = File::new(Some(format!("proof_{}.dot", current_time_str)));
            self.file_dot.create();
            self.file_dot.writeln("digraph {");
        }
        if self.print_txt_proof {
            self.file_txt = File::new(Some(format!("proof_{}.txt", current_time_str)));
            self.file_txt.create();
        }
        if self.print_tex_proof {
            self.file_tex = File::new(Some(format!("proof_{}.tex", current_time_str)));
            self.file_tex.create();
            self.file_tex.writeln("\\documentclass{article}");
            self.file_tex.writeln("\\usepackage{seqsplit}\\usepackage{mathtools}\\usepackage{amssymb}");
            self.file_tex.writeln("\\newcommand{\\overflow}[1]{ \\texttt{\\ttfamily\\seqsplit{$#1$}} }");
            self.file_tex.writeln("\\begin{document}");
            self.file_tex.writeln("\\title{Proof of unsatisfiability}");
            self.file_tex.writeln("\\author{generated by SAT solver}");
            self.file_tex.writeln(format!("\\date{{{}}}", current_time.format("%Y %B %d %H:%M:%S")).as_str());
            self.file_tex.writeln("\\maketitle");
            self.file_tex.write("\\overflow{\\emptyset||F} ");
        }
    }

    /// Close the proof files.
    fn file_close(&mut self) {
        if self.print_dot_proof {
            self.file_dot.writeln("}");
        }
        if self.print_tex_proof {
            self.file_tex.writeln("\\end{document}");
        }
    }

    /// Delete the proof files.
    /// It is called only when the formula is satisfiable.
    fn file_delete(&mut self) {
        if self.print_dot_proof {
            self.file_dot.delete();
        }
        if self.print_txt_proof {
            self.file_txt.delete();
        }
        if self.print_tex_proof {
            self.file_tex.delete();
        }
    }

    /// Print the statistics of the solver.
    pub fn print_stats(&self) {
        println!("Clauses learned: {}", self.stats.get_clauses_learned());
        println!("Clauses forgotten: {}", self.stats.get_clauses_forgotten());
        println!("Max virtual memory: {}", self.stats.get_virtual_memory());
        println!("Max physical memory: {}", self.stats.get_physical_memory());
    }

}