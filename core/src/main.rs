use colored::*;
use omicron;
fn main() {
    let user_id = omicron::users::UserId(50);
    println!("user_id: {:?}", user_id);
    println!("{}", "Initialzing services...".green().bold());
}
