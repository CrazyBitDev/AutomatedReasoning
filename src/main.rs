pub mod input;
pub mod files;
pub mod tools;

pub mod consts;
pub mod classes;

use std::vec;
use std::time::Instant;

pub use crate::classes::solver::Solver;
pub use crate::consts::{sat::SAT, editor_types::EditorTypes};


fn main() {
    
    let mut solver = Solver::new();

    println!("");
    println!("        SAT  Solver        ");
    println!("A program by Matteo Ingusci");
    println!("---------------------------");
    println!("UniVR - Automated Reasoning");
    println!("      A.Y.  2023/2024      ");
    println!("");

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        //check if an argument is a file
        for arg in args.iter().skip(1) {
            if files::file_exists(arg) && arg.ends_with(".cnf") {
                match solver.formula.load_file(arg) {
                    Ok(()) => println!("File loaded successfully!"),
                    Err(e) => eprintln!("Error loading file: {:?}", e),
                }
            } //else if is "-dot"
            else if arg == "-dot" {
                solver.set_dot_proof_enabled(true);
                println!("Dot proof file enabled.")
            } //else if is "-txt"
            else if arg == "-txt" {
                solver.set_txt_proof_enabled(true);
                println!("Txt proof file enabled.")
            } //else if is "-tex"
            else if arg == "-tex" {
                solver.set_tex_proof_enabled(true);
                println!("Tex proof file enabled.")
            }
        }
        if solver.is_formula_loaded() {
            let start = Instant::now();
            match solver.solve() {
                Ok(sat) => {
                    match sat {
                        SAT::Satisfiable => {
                            println!("The formula is satisfiable!");
                            println!("The following model satisfies the formula:");
                            solver.print_model();
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
                },
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
            return ();
        }
    }
    
    loop {

        let mut choices: Vec<&str> = Vec::new();

        if solver.is_formula_loaded() {
            choices.push("Solve");
            choices.push("Solver options");
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
                            "DIMACS",
                            "Back"
                        ]) {
                            Ok(choice) => {
                                if choice == "DIMACS" {
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
                                    println!("The following model satisfies the formula:");
                                    solver.print_model();
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
                } else if choice == "Solver options" {
                    match input::editor_menu(
                        vec![
                            "Change the solver options:"
                        ],
                        vec![
                            ("Print .dot proof file", EditorTypes::Bool(solver.is_dot_proof_enabled())),
                            ("Print .txt proof file", EditorTypes::Bool(solver.is_txt_proof_enabled())),
                            ("Print .tex proof file", EditorTypes::Bool(solver.is_tex_proof_enabled()))
                        ]
                    ) {
                        Ok(results) => {
                            for result in results {
                                match result {
                                    ("Print .dot proof file", enabled) => {
                                        solver.set_dot_proof_enabled(enabled == 1);
                                    },
                                    ("Print .txt proof file", enabled) => {
                                        solver.set_txt_proof_enabled(enabled == 1);
                                    },
                                    ("Print .tex proof file", enabled) => {
                                        solver.set_tex_proof_enabled(enabled == 1);
                                    },
                                    _ => (),
                                };
                            }
                        },
                        Err(_e) => (),
                    }
                } else if choice == "Print" {
                    //solver.formula.print_stats();

                    match input::choice_menu(
                        vec![
                            format!("Number of variables: {}", &solver.formula.get_num_variables()).as_str(),
                            format!("Number of clauses: {}", &solver.formula.get_num_clauses()).as_str(),
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