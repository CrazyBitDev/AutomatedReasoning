use crate::classes::formula_node::FormulaNode;

pub fn clear_parenthesis(formula: &mut FormulaNode) {
    formula.iter_child(|node| {
        while node.children_len() == 1 && !node.get_child(0).unwrap().is_literal() {
            println!("Children len: {}", node.children_len());
            println!("Before: {}", node);
            let child = node.get_child(0).unwrap().clone();
            node.set_children(child.get_children(Option::None, Option::None));
            println!("After: {}", node);
        }
    }, true);
    if formula.children_len() == 1 {
        let child = formula.get_child(0).unwrap().clone();
        formula.set_children(child.get_children(Option::None, Option::None));
    }
}