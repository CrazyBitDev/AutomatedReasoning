use std::collections::HashMap;
use std::cmp;

pub struct Clause {
    pub literals: Vec<i32>,
    pub is_satisfied: bool,
    pub is_always_satisfied: bool,

    pub two_watched_literals: (usize, usize),
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            literals: Vec::new(),
            is_satisfied: false,
            is_always_satisfied: false,

            two_watched_literals: (0, 0),
        }
    }

    pub fn load_string(&mut self, literal_string: String) -> Result<(), ()> {

        if literal_string.trim().is_empty()
          || literal_string.trim().chars().next().unwrap().is_alphabetic()
          || literal_string.trim() == "0"
          || literal_string.trim() == "%" {
            return Err(());
        }

        self.literals = literal_string
            .split_whitespace()
            .filter_map(|literal| literal.parse().ok())
            .filter(|&literal| literal != 0)
            .collect();
        self.check_literals();

        Ok(())
    }

    pub fn load_vec(&mut self, literals: Vec<i32>) {
        self.literals = literals;
        self.check_literals();
    }

    pub fn add_literal(&mut self, literal: i32) {
        self.literals.push(literal);
        self.check_literals();
    }
    pub fn remove_literal(&mut self, literal: i32) {
        self.literals.retain(|&x| x != literal);
        self.check_literals();
    }

    fn is_unit_clause(&self) -> bool {
        self.two_watched_literals.0 == self.two_watched_literals.1
    }

    fn get_watched_literal_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            return self.two_watched_literals.0;
        } else {
            return self.two_watched_literals.1;
        }
    }
    fn set_watched_literal_idx(&mut self, idx: usize, value: usize) {
        if idx == 0 {
            self.two_watched_literals.0 = value;
        } else {
            self.two_watched_literals.1 = value;
        }
    }

    fn check_literals(&mut self) {
        // if the clause contains a literal and its negation, the clause is always satisfied
        for literal in &self.literals {
            if self.literals.contains(&-literal) {
                self.is_always_satisfied = true;
                break;
            }
        }
        if self.literals.len() > 1 {
            self.two_watched_literals = (0, 1)
        }
    }

    fn check_literal_is_satisfied(&mut self, literal_idx: usize, instance: &mut Vec<i32>) -> Result<bool, ()> {

        if instance.contains(&self.literals[literal_idx]) {
            return Ok(true);
        } else if instance.contains(&(self.literals[literal_idx] * -1)) {
            return Err(()); // conflict
        }

        return Ok(false);
    }

    pub fn check_satisfied(&mut self, instance: &mut Vec<i32>, force_check: bool) -> Result<bool, ()> {
        if self.is_always_satisfied || (self.is_satisfied && !force_check) {
            return Ok(true);
        }

        // two-watched literal propagation
        if self.is_unit_clause() {
            match self.check_literal_is_satisfied(self.get_watched_literal_idx(0), instance) {
                Ok(is_satisfied) => {
                    self.is_satisfied = is_satisfied;
                    return Ok(is_satisfied);
                },
                Err(()) => {
                    return Err(());
                }
            }
        } else {
            'two_watched_literals_loop: loop {
                for i in 0..2 {
                    match self.check_literal_is_satisfied(self.get_watched_literal_idx(i), instance) {
                        Ok(is_satisfied) => {
                            if is_satisfied {
                                self.is_satisfied = true;
                                return Ok(true);
                            }
                        },
                        Err(()) => {
                            
                            let max = cmp::max(self.get_watched_literal_idx(0), self.get_watched_literal_idx(1));
                            if max == self.literals.len() - 1 {
                                return Err(());
                            } else {
                                self.set_watched_literal_idx(i, max + 1);
                                continue 'two_watched_literals_loop;
                            }
                            
                        }
                    }
                }
                break;
            }
            return Ok(false);
        }
    }

    pub fn print(&self) {
        let literals_str: Vec<String> = self.literals.iter().map(|&literal| literal.to_string()).collect();
        println!("{}", literals_str.join(" "));
    }

    fn print_with_chars(&self, literal_map: HashMap<u32, String>) {
        let literals_str: Vec<String> = self.literals
            .iter()
            .map(|&literal| literal_map.get(&(literal.abs() as u32)).unwrap().clone())
            .collect();
        println!("{}", literals_str.join(" "));
    }
}