use crate::classes::formula_node::FormulaNode;
use crate::consts::operators;

pub fn iff_solver(formula: &mut FormulaNode) {
    formula.find_node_with_child(
        |granchild| {
            granchild.get_operator() == operators::IFF
        },
        |parent, child_idx| {
            let left_children = parent.get_children(Option::None, Some(child_idx));
            let right_children = parent.get_children(Some(child_idx + 1), Option::None);

            parent.set_children(vec![
                FormulaNode::new_by_children(vec![
                    FormulaNode::new_by_children(left_children.clone(), false),
                    FormulaNode::new_by_operator(operators::OR.to_string()),
                    FormulaNode::new_by_children(right_children.clone(), true),
                ], false),
                FormulaNode::new_by_operator(operators::AND.to_string()),
                FormulaNode::new_by_children(vec![
                    FormulaNode::new_by_children(left_children, true),
                    FormulaNode::new_by_operator(operators::OR.to_string()),
                    FormulaNode::new_by_children(right_children, false),
                ], false),
            ])
        },
        true
    );
}

pub fn if_solver(formula: &mut FormulaNode) {
    formula.find_node_with_child(
        |granchild| {
            granchild.get_operator() == "→"}
        ,
        |parent, child_idx| {
            let left_children = parent.get_children(Option::None, Some(child_idx));
            let right_children = parent.get_children(Some(child_idx + 1), Option::None);

            parent.set_children(vec![
                FormulaNode::new_by_children(vec![
                    FormulaNode::new_by_children(left_children, true),
                    FormulaNode::new_by_operator(operators::OR.to_string()),
                    FormulaNode::new_by_children(right_children, false),
                ], false),
            ])
        },
        true
    );
}

pub fn not_solver(formula: &mut FormulaNode) {
    formula.iter_child(|parent| {
        if parent.is_negated() && parent.children_len() > 0 {
            parent.iter_child(|child| {
                child.toggle_negated();
            }, false);
            parent.toggle_negated();
        }
    }, true)
}

pub fn distributivity_solver(formula: &mut FormulaNode) -> FormulaNode {
    let mut formula = formula.clone();
    if formula.children_len() > 0 {
        'iter_parent: loop {
            let mut operator = "".to_string();
            for child_idx in 0..formula.children_len() {
                if let Ok(child) = formula.get_child(child_idx) {
                    if child.is_operator() {
                        if operator.len() == 0 {
                            operator = child.get_operator();
                        } else if operator != child.get_operator() {
                            if let Ok(next_child) = formula.get_child(child_idx + 1) {
                                let mut cloned_formula = formula.clone();
                                'child_loop: for j in 0..child_idx {
                                    let j_child = formula.get_child(j).unwrap();
                                    if j_child.is_operator() {
                                        continue 'child_loop;
                                    }
                                    let mut new_child = FormulaNode::new_by_children(vec![], false);
                                    new_child.append_child(j_child.clone());
                                    new_child.append_child(FormulaNode::new_by_operator(child.get_operator()));
                                    new_child.append_child(next_child.clone());

                                    cloned_formula.set_child(new_child, j);
                                }
                                cloned_formula.remove_child(child_idx);
                                cloned_formula.remove_child(child_idx);
                                formula = cloned_formula;
                            } else {
                                panic!("Error, operator without right child");
                            }
                            continue 'iter_parent;
                        }
                    }
                }
            }
            break;
        }
    }
    println!("Pending: {}", formula);
    return formula;
}