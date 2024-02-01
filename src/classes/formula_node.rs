use std::fmt;
use std::collections::HashMap;

use crate::consts::operators;

#[derive(Clone)]
pub struct FormulaNode {
    children: Vec<FormulaNode>,

    literal: i32,
    operator: String,

    negated: bool,
}

impl FormulaNode {
    pub fn new(subformula: Vec<String>, parenthesis: HashMap<usize, usize>, negated: bool) -> FormulaNode {
        let mut formula_node = FormulaNode {
            children: Vec::new(),
            literal: 0,
            operator: String::new(),
            negated: negated,
        };

        if subformula.len() == 1 {
            let element = &subformula[0];
            if element == "→" || element == "←" || element == "∧" || element == "∨" {
                formula_node.operator = element.to_string();
            } else {
                formula_node.literal = match element.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => 0,
                };
            }
        } else {

            let mut idx = 0;
            let mut negation = false;

            while idx < subformula.len() {

                let element = &subformula[idx];

                if element == "-" {
                    negation = true;
                } else if element == "(" {
                    let mut parenthesis_clone = parenthesis.clone();
                    parenthesis_clone.retain(|k, _v| 
                        k != &idx && k >= &((&idx + 1))    
                    );
                    parenthesis_clone = parenthesis_clone
                        .iter()
                        .map(|(k, v)|
                            (k - idx - 1, v - idx - 1)
                        )
                        .collect::<HashMap<usize, usize>>();
                    formula_node.children.push(FormulaNode::new(
                        subformula[idx + 1..*parenthesis.get(&idx).unwrap()].to_vec(),
                        parenthesis_clone,
                        negation)
                    );
                    negation = false;
                    idx = *parenthesis.get(&idx).unwrap();
                } else if element == "→" || element == "←" || element == "∧" || element == "∨" {
                    formula_node.children.push(FormulaNode::new(vec![element.to_string()], parenthesis.clone(), negation));
                } else {
                    formula_node.children.push(FormulaNode::new(vec![element.to_string()], parenthesis.clone(), negation));
                    negation = false;
                }

                idx += 1;
            }
        }

        return formula_node;
    }

    pub fn new_by_children(childrens: Vec<FormulaNode>, negated: bool) -> FormulaNode {
        FormulaNode {
            children: childrens,
            literal: 0,
            operator: String::new(),
            negated: negated,
        }
    }

    pub fn new_by_literal(literal: i32, negated: bool) -> FormulaNode {
        FormulaNode {
            children: Vec::new(),
            literal: literal,
            operator: String::new(),
            negated: negated,
        }
    }

    pub fn new_by_operator(operator: String) -> FormulaNode {
        FormulaNode {
            children: Vec::new(),
            literal: 0,
            operator: operator,
            negated: false,
        }
    }

    pub fn is_literal(&self) -> bool {
        return self.literal > 0;
    }

    pub fn is_operator(&self) -> bool {
        return self.operator != "";
    }

    pub fn get_operator(&self) -> String {
        return self.operator.clone();
    }

    pub fn set_operator(&mut self, operator: String) {
        self.operator = operator;
    }

    pub fn children_len(&self) -> usize {
        return self.children.len();
    }

    pub fn get_children(&self, from: Option<usize>, to: Option<usize>) -> Vec<FormulaNode> {
        let mut children = self.children.clone();
        if let Some(from) = from {
            children = children[from..].to_vec();
        }
        if let Some(to) = to {
            children = children[..to].to_vec();
        }
        return children;
    }

    pub fn set_children(&mut self, children: Vec<FormulaNode>) {
        self.children = children;
    }

    pub fn get_child(&self, child_idx: usize) -> Result<&FormulaNode, ()> {
        if self.children.len() > 0 && child_idx < self.children.len() {
            return Ok(&self.children[child_idx]);
        }
        return Err(());
    }
    pub fn get_mut_child(&mut self, child_idx: usize) -> Result<&mut FormulaNode, ()> {
        if self.children.len() > 0 && child_idx < self.children.len() {
            return Ok(&mut self.children[child_idx]);
        }
        return Err(());
    }

    pub fn set_child(&mut self, child: FormulaNode, child_idx: usize) {
        if self.children.len() > 0 && child_idx < self.children.len() {
            self.children[child_idx] = child;
        }
    }
    pub fn append_child(&mut self, child: FormulaNode) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child_idx: usize) {
        if self.children.len() > 0 && child_idx < self.children.len() {
            self.children.remove(child_idx);
        }
    }

    pub fn is_negated(&self) -> bool {
        return self.negated;
    }

    pub fn set_negated(&mut self, negated: bool) {
        self.negated = negated;
    }

    pub fn toggle_negated(&mut self) {
        if self.is_operator() {
            if self.operator == operators::AND {
                self.operator = operators::OR.to_string();
            } else if self.operator == operators::OR {
                self.operator = operators::AND.to_string();
            }
        } else {
            self.negated = !self.negated;
        }
    }

    pub fn iter_child(&mut self, function: fn(&mut FormulaNode), deep: bool) {
        if self.children.len() > 0 {
            for child in &mut self.children {
                function(child);
                if deep {
                    child.iter_child(function, deep);
                };
            }
        }
    }

    pub fn find_node_with_child(&mut self, search_fn: fn(&FormulaNode) -> bool, found_fn: fn(&mut FormulaNode, usize), deep: bool) {
        if self.children.len() > 0 {
            for (idx, child) in self.children.iter_mut().enumerate() {
                if child.literal > 0 || child.operator != "" {
                    if search_fn(child) {
                        return found_fn(self, idx);
                    }
                } else if deep {
                    child.find_node_with_child(search_fn, found_fn, deep);
                }
            }
        }
    }
}

impl fmt::Display for FormulaNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.children.len() == 0 {
            if self.literal > 0 {
                if self.negated {
                    return write!(f, "-{}", self.literal);
                } else {
                    return write!(f, "{}", self.literal);
                }
            } else {
                return write!(f, "{}", self.operator);
            }
        } else {
            let mut result = String::new();
            if self.negated {
                result.push_str("-");
            }
            result.push_str("(");
            for (_i, child) in self.children.iter().enumerate() {
                result.push_str(&format!("{}", child));
            }
            result.push_str(")");
            write!(f, "{}", result)
        }
    }
}
