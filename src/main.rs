pub mod classes;
pub mod input;
pub mod files;

pub use crate::classes::formula::Formula;

fn main() {

    println!("\n        SAT  Solver        ");
    println!("A program by Matteo Ingusci");
    println!("---------------------------");
    println!("UniVR - Automated Reasoning");
    println!("      A.Y.  2023/2024      \n");

    //let data = input::input_formatted().unwrap();
    /*let data = files::read_file("C:\\Users\\matte\\Desktop\\AutomatedReasoning\\test\\uf20-01.cnf");
    match data {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error reading file: {:?}", e),
    }*/

    /*let mut formula = Formula::new();
    formula.load_file("C:\\Users\\matte\\Desktop\\AutomatedReasoning\\test\\uf20-01.cnf");*/
    
}