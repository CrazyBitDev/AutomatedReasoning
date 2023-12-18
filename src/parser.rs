pub fn parser(formula_string: Vec<String>) -> Result<Formula, io::Error> {
    let mut formula = formula.clone();

    formula = match cleaner(formula) {
        Ok(cleaned_formula) => cleaned_formula,
        Err(e) => return Err(e),
    };

    let parenthesis = match parenthesis_check(formula) {
        Ok(parenthesis) => parenthesis,
        Err(e) => return Err(e),
    };

    match syntax_check(formula) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }
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

fn parenthesis_check(formula: Vec<String>) -> Result<Vec<(usize, usize)>, io::Error> {

    let mut parenthesis: Vec<(usize, usize)> = Vec::new();
    let mut parenthesis_queue: Vec<usize> = Vec::new();
    
    for (idx, element) in formula.iter().enumerate() {
        if element == "(" {
            parenthesis_queue.push(idx);
        } else if element == ")" {
            
            let open_position = match parenthesis_queue.pop() {
                Some(open_position) => open_position,
                None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"))
            };

            if idx - open_position == 1 {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"));
            }

            parenthesis.push((open_position, idx));
        }
    }

    if parenthesis_queue.len() > 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parenthesis error"));
    }

    return Ok(parenthesis);
}

fn syntax_check(formula: Vec<String>) -> Result<(), io::Error> {

}