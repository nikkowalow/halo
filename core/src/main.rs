use colored::*;
use mu;
use omicron;
use std::thread;

fn main() {
    println!("{}", "Initializing services...".green().bold());

    let mu_thread = thread::spawn(|| {
        mu::run().expect("mu failed");
    });

    let omicron_thread = thread::spawn(|| {
        omicron::run().expect("omicron failed");
    });

    mu_thread.join().unwrap();
    omicron_thread.join().unwrap();

    println!("{}", "shutting down system.".red().bold());
}
