use rustic::parser::*;
use rustic::visitors::*;
use rustic::visitors::visitor::NodeAccept;
use crate::variable_checker::*;
use crate::constprop::*;

use clap::{Arg, Command};
use std::fs;

fn main() {
    // Define the command-line interface using `clap`
    let matches = Command::new("rustic")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Konstantinos Fragkiadakis, fconstantinos@gmail.com")
        .about("A parser for a small subset of the Rust programming language")
        .arg(
            Arg::new("file")
                .help("The input file to use")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("constprop")
                .long("constprop")
                .required(false)
                .action(clap::ArgAction::SetTrue)
                .help("Enable constant propagation optimization")
        )
        .get_matches();

    // Get the file name from the command-line arguments
    let file_name = matches.get_one::<String>("file").expect("required argument");

    // Fetch file string.
    let unparsed_file = fs::read_to_string(file_name).expect("cannot read file");
    println!("Unparsed file:\n{:?}\n", unparsed_file);

    // Create AST from file string.
    let mut file = parse(&unparsed_file).expect("unsuccessful parse");

    // Check for undefined variables and redefinitions.
    let mut variable_checker = VariableChecker::new();
    file.accept(&mut variable_checker);

    // Perform constant folding only if the --constprop flag is set.
    if matches.contains_id("constprop") && matches.get_flag("constprop") {
        let mut constant_propagation = ConstantPropagation::new();
        file.accept(&mut constant_propagation);
    }

    // Write program to output.
    println!("Resulting program:\n\n{}", file);
}
