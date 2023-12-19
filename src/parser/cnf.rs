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