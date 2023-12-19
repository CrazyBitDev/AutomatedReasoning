use std::any::Any;
use std::io;
use std::collections::{BTreeSet, HashMap};
use crate::classes::formula::Formula;
use crate::classes::formula_node::FormulaNode;

use crate::consts::operators;

mod cnf;
mod tools;

pub fn parse(formula_param: Vec<String>) -> Result<Formula, io::Error> {
    let mut formula = formula_param.clone();

    formula = match cleaner(formula) {
        Ok(cleaned_formula) => cleaned_formula,
        Err(e) => return Err(e),
    };

    formula = merge_literals(formula);

    let (
        parenthesis,
        variables,
    ) = match check_syntax(&formula) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    let mut formulaObj = Formula::new();
    formulaObj.formula_string = formula.clone();

    let mut literal_map: HashMap<usize, String> = HashMap::new();

    (
        formula,
        literal_map
    ) = match parse_variables(formula, variables) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    formulaObj.literal_map = literal_map;

    let mut formula_node = match parse_syntax(formula, parenthesis) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    cnf_converter(&mut formula_node);
    
    println!("Formula: {}", formula_node);

    Ok(formulaObj)
}

fn cleaner(mut formula: Vec<String>) -> Result<Vec<String>, io::Error> {
    //trim all the elements
    formula = formula.iter().map(|x| x.trim().to_string()).collect();

    //remove all empty elements
    formula.retain(|x| x.len() > 0);

    //if the formula is empty, return an error
    if formula.len() == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty formula"));
    }

    return Ok(formula);
}

fn merge_literals(mut formula: Vec<String>) -> Vec<String> {

    let mut last_literal_pos = 0;
    let mut idx = 0;

    //for each element of the formula
    while idx < formula.len() {
        let element = &formula[idx];
        //if the element is a literal
        if element != "(" && element != ")" && element != "→" && element != "←" && element != "∧" && element != "∨" && element != "-" {
            //if the last element was a literal
            if last_literal_pos != 0 {
                //merge the two literals
                formula[last_literal_pos] = format!("{}{}", formula[last_literal_pos], element);
                //remove the current literal
                formula.remove(idx);
                continue;
            } else {
                //set the last literal position
                last_literal_pos = idx;
            }
        } else {
            //reset the last literal position
            last_literal_pos = 0;
        }
        idx += 1;
    }

    return formula;
}

fn check_syntax(formula: &Vec<String>) -> Result<(HashMap<usize, usize>, BTreeSet<String>), io::Error> {

    let mut parenthesis: HashMap<usize, usize> = HashMap::new();
    let mut parenthesis_queue: Vec<usize> = Vec::new();

    let mut variables: BTreeSet<String> = BTreeSet::new();
    
    let mut is_start = true;

    let mut check_map: HashMap<&str, bool> = HashMap::new();
    check_map.insert("negation", false);
    check_map.insert("open_parenthesis", false);
    check_map.insert("close_parenthesis", false);
    check_map.insert("operator", false);
    check_map.insert("literal", false);
    check_map.insert("if", false);
    check_map.insert("iff", false);
    let mut last_check = "";

    for (idx, element) in formula.iter().enumerate() {
        if element == "(" {
            if !is_start && (
                *check_map.get("close_parenthesis").unwrap() ||
                *check_map.get("literal").unwrap()
            ) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            parenthesis_queue.push(idx);

            last_check = "open_parenthesis";
        } else if element == ")" {
            if is_start ||
                *check_map.get("negation").unwrap() ||
                *check_map.get("open_parenthesis").unwrap() ||
                *check_map.get("operator").unwrap() ||
                *check_map.get("if").unwrap() ||
                *check_map.get("iff").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            let open_position = match parenthesis_queue.pop() {
                Some(open_position) => open_position,
                None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"))
            };

            if idx - open_position == 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"));
            }

            parenthesis.insert(open_position, idx);
            
            last_check = "close_parenthesis";
        } else if element == "→" {
            if is_start ||
                *check_map.get("negation").unwrap() ||
                *check_map.get("open_parenthesis").unwrap() ||
                *check_map.get("operator").unwrap() ||
                *check_map.get("if").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            if !check_map.get("iff").unwrap() {
                last_check = "if";
            }
        } else if element == "←" {
            if is_start ||
                *check_map.get("negation").unwrap() ||
                *check_map.get("open_parenthesis").unwrap() ||
                *check_map.get("operator").unwrap() ||
                *check_map.get("if").unwrap() ||
                *check_map.get("iff").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }
            last_check = "iff";
        } else if element == "-" {
            if *check_map.get("close_parenthesis").unwrap() ||
                *check_map.get("literal").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            last_check = "negation";
        } else if element == "∧" || element == "∨" {
            if is_start ||
                *check_map.get("negation").unwrap() ||
                *check_map.get("open_parenthesis").unwrap() ||
                *check_map.get("operator").unwrap() ||
                *check_map.get("if").unwrap() ||
                *check_map.get("iff").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            last_check = "operator";
        } else {
            if *check_map.get("literal").unwrap() ||
                *check_map.get("close_parenthesis").unwrap()
            {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
            }

            variables.insert(element.to_string());

            last_check = "literal";
        }

        is_start = false;
        for value in check_map.values_mut() {
            *value = false;
        }
        check_map.insert(last_check, true);
    }

    if parenthesis_queue.len() > 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"));
    }

    if variables.len() == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty formula"));
    }

    return Ok((parenthesis, variables));
}

fn parse_variables(mut formula: Vec<String>, variables: BTreeSet<String>) -> Result<(Vec<String>, HashMap<usize, String>), io::Error> {
    let literal_map: HashMap<usize, String> = variables.iter().enumerate().map(|(i, v)| (i + 1, v.clone())).collect::<HashMap<_, _>>();

    for element in formula.iter_mut() {
        if element != "(" && element != ")" && element != "→" && element != "←" && element != "∧" && element != "∨" && element != "-" {
            //get key from value
            let literal = match literal_map.iter().find(|(_key, value)| *value == element) {
                Some((key, _value)) => key,
                None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Literal not found")),
            };
            *element = literal.to_string();
        }
    }

    return Ok((formula, literal_map));
}

fn parse_syntax(formula: Vec<String>, parenthesis: HashMap<usize, usize>) -> Result<FormulaNode, io::Error> {

    let mut formula_node = FormulaNode::new(formula, parenthesis, false);

    formula_node.find_node_with_child(
        |child| {
            child.get_operator() == operators::LEFT_ARROW
        },
        |parent, child_idx| {
            let child = parent.get_child(child_idx).unwrap();
            child.set_operator(operators::IFF.to_string());
            parent.remove_child(child_idx+1); 
        }, 
        true
    );

    return Ok(formula_node);
}

fn cnf_converter(formula: &mut FormulaNode) {
    cnf::iff_solver(formula);
    cnf::if_solver(formula);
    cnf::not_solver(formula);
    tools::clear_parenthesis(formula);
}