use colored::*;
use mu;
use omicron;

fn main() {
    println!("{}", "Initialzing services...".green().bold());
    mu::run().expect("mu failed");
    omicron::run().expect("omicron failed");
}
