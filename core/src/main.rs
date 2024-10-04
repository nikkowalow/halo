use colored::*;
use omicron;
fn main() {
    omicron::run().expect("omicron failed");

    let user_id = omicron::users::UserId(50);
    println!("user_id: {:?}", user_id);
    println!("{}", "Initialzing services...".green().bold());
}
