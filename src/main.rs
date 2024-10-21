mod actor;

use ractor::Actor;
use crate::actor::{CrossTerm, Ratatui};

#[tokio::main]
async fn main() {

    let (terminal, _term_handle) =
        Actor::spawn(None, Ratatui, ()).await.expect("failed to create terminal");

    // Build an ActorRef along with a JoinHandle which lives for the life of the 
    // actor. Most of the time we drop this handle, but it's handy in the 
    // main function to wait for clean actor shut-downs (all stop handlers will
    // have completed)
    let (_crossterm, crossterm_handle) = 
        Actor::spawn(None, CrossTerm, ())
            .await
            .expect("Failed to connect to Terminal event listener");

    // Cleanup
    crossterm_handle.await.unwrap();
}
