mod actor;

use kameo;
use crate::actor::{CrossTerm, crossterm::Read};

#[tokio::main]
async fn main() {

    let crossterm = kameo::spawn(CrossTerm::new());
    crossterm.wait_for_stop().await;
    println!("Goodbye.");
}
