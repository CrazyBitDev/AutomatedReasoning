pub mod classes;
pub mod consts;

pub mod input;
pub mod files;
pub mod parser;
pub mod tools;

use std::vec;

pub use crate::classes::solver::Solver;
pub use crate::consts::sat::SAT;

use std::time::{Duration, Instant};

fn main() {

    let mut solver = Solver::new();

    println!("");
    println!("        SAT  Solver        ");
    println!("A program by Matteo Ingusci");
    println!("---------------------------");
    println!("UniVR - Automated Reasoning");
    println!("      A.Y.  2023/2024      ");
    println!("");
    
    loop {

        let mut choices: Vec<&str> = Vec::new();

        if solver.is_formula_loaded() {
            choices.push("Solve");
            choices.push("Print");
            choices.push("Clear formula");
        } else {
            choices.push("Load CNF file");
            choices.push("Write the formula");
        }

        choices.push("Exit");
        
        match input::choice_menu(vec![
            "",
            "        SAT  Solver        ",
            "A program by Matteo Ingusci",
            "---------------------------",
            "UniVR - Automated Reasoning",
            "      A.Y.  2023/2024      ",
            ""
        ], choices) {
            Ok(choice) => {
                if choice == "Load CNF file" {
                    match input::input("Insert the path of the CNF file: ") {
                        Ok(path) => match solver.formula.load_file(&path) {
                            Ok(()) => println!("File loaded successfully!"),
                            Err(e) => eprintln!("Error loading file: {:?}", e),
                        },
                        Err(e) => eprintln!("Error: {:?}", e),
                    }
                    input::pause(Option::None);
                } else if choice == "Write the formula" {
                    
                    match input::choice_menu(
                        vec![
                            "Select the formula format:",
                        ], vec![
                            "PL formula",
                            "DIMACS",
                            "Back"
                        ]) {
                            Ok(choice) => {
                                if choice == "PL formula" {
                                    println!("To write the formula in PL format, you have to write the entire formula in a single line.");
                                    println!("You can choose to write the literals as numbers, or as characters.");
                                    println!("If you choose to write the literals as characters, keep in mind that the characters are case sensitive.");
                                    println!("The literals can be separated by spaces, but it's not necessary.");
                                    println!("\n");
                                    println!("Between the literals, you can use the following operators:");
                                    println!("∧ (AND): ctrl-a, *, &");
                                    println!("∨ (OR): ctrl-o, +, |");
                                    println!("¬ (NOT): ctrl-n, -");
                                    println!("→ (IMPLIES): ctrl-i, ->");
                                    println!("↔ (IFF): ctrl-f, <->");
                                    println!("Keep in mind that you can use the parenthesis: (, )");
                                    println!("\n");
                                    println!("Once you have written the formula, you can press enter to continue.");

                                    match input::input_formatted() {
                                        Ok(formula) => {
                                            if formula.len() > 0 {

                                                match parser::parse(formula) {
                                                    Ok(formulaObj) => {
                                                        //println!("Formula: {}", formulaObj.formula_string.join(""));
                                                    },
                                                    Err(e) => {
                                                        eprintln!("Error: {:?}", e);
                                                    }
                                                }
                                            } else {
                                                eprintln!("Formula: <empty>");
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    };


                                } else if choice == "DIMACS" {
                                    println!("To write the formula in DIMACS format, you have to write a clauses one per line.");
                                    println!("You need to write the literals as numbers separated by spaces, and the clauses must be separated by newlines.");
                                    println!("In case of a negative literal, you have to write the minus sign before the literal.");
                                    println!("To end the formula, or to abort the input process, you have to write a line with the single character '0'.");
                                    println!("\n");
                                    println!("For example, the formula (a ∨ b) ∧ (¬a ∨ ¬b) can be written as follows:");
                                    println!("1 2");
                                    println!("-1 -2");
                                    println!("0");

                                    let mut counter = 1;

                                    loop {
                                        match input::input(format!("Insert the clause n° {}:", counter).as_str()) {
                                            Ok(clause) => {
                                                if clause.trim() == "0" {
                                                    break;
                                                }

                                                match solver.formula.add_clause_by_string(clause) {
                                                    Ok(()) => {
                                                        counter += 1;
                                                    },
                                                    Err(()) => {
                                                        eprintln!("Error, retry.");
                                                        input::pause(Option::None);
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                eprintln!("Error: {:?}", e);
                                                input::pause(Option::None);
                                            }
                                        }
                                    }
                                    solver.formula.calculate_stats();
                                }

                                if choice != "Back" {
                                    input::pause(Option::None);
                                }
                            },
                            Err(_e) => (),
                        }
                } else if choice == "Clear formula" {
                    solver.reset();
                } else if choice == "Solve" {
                    solver.reset_solve();
                    let start = Instant::now();
                    match solver.solve() {
                        Ok(sat) => {
                            match sat {
                                SAT::Satisfiable => {
                                    println!("The formula is satisfiable!");
                                    println!("The following instance satisfies the formula:");
                                    solver.print_instance();
                                },
                                SAT::Unsatisfiable => {
                                    println!("The formula is unsatisfiable!");
                                },
                                SAT::Unknown => {
                                    println!("The formula is unknown!");
                                },
                            }
                            println!("Time elapsed in is: {:?}", start.elapsed());
                            solver.print_stats();
                            input::pause(Option::None);
                        },
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                } else if choice == "Print" {
                    //solver.formula.print_stats();

                    match input::choice_menu(
                        vec![
                            format!("Number of variables: {}", &solver.formula.num_variables).as_str(),
                            format!("Number of clauses: {}", &solver.formula.num_clauses).as_str(),
                            "",
                            "Select the print mode:"
                        ],
                        vec![
                            "Print in CNF format",
                            "Print in DIMACS format",
                            "Back"
                        ]
                    ) {
                        Ok(choice) => {
                            if choice == "Print in CNF format" {
                                solver.formula.print_cnf();
                            } else if choice == "Print in DIMACS format" {
                                solver.formula.print_dimacs();
                            }
                            
                            if choice != "Back" {
                                input::pause(Option::None);
                            }
                        },
                        Err(_e) => (),
                    }

                } else if choice == "Exit" {
                    break;
                }
            },
            Err(_e) => break,
        }
    }

    println!("Bye!");
    
}