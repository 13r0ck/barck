use std::io::stdout;
use crossterm::{
    execute,
    event::{Event, EventStream},
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use ractor::{cast, Actor, ActorRef, ActorProcessingErr, RactorErr};
use futures_util::{StreamExt};

/// Crossterm is a cross platform (Operating System) abstration for
/// terminal manupulation and event system.
/// All this actor does is setup the terminal and listen for events untill it is killed.
/// Start the actor, then send a `StartListen` message.
/// Always send a StopListen command when done, even on error, restart the actor and
/// send a StopListen even if you intend to not keep using the actor.
pub struct CrossTerm;
pub enum Msg {
    /// Enables terminal raw mode, and starts listening to terminal events.
    StartListen,
    /// Disables terminal raw mode, and stops the event listener loop.
    StopListen,
    /// An internal message type to this actor, essentially a `loop {}`
    Read,
    Event(Event),
}

#[derive(Debug)]
enum Error {
    /// [`Msg::Event`] should not loop back into this type.
    LoopedBack(Event),
    Ractor(RactorErr<Msg>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let res = match self {
            Error::LoopedBack(_) => "A event was passed back into the actor. This actor should only produce messages.",
            Error::Ractor(_) => "ractor error",
            
        };
        write!(f, "{res}")
    }
}

// TODO implement this better
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        ""
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

#[async_trait::async_trait]
impl Actor for CrossTerm {
    type State = EventStream;
    type Msg = Msg;
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _arguments: Self::Arguments) -> Result<Self::State, ActorProcessingErr> {
        Ok(EventStream::new())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            Msg::Read => {
                let event = state.next().await;
                cast!(myself, Msg::Read)
                    .map_err(|e| Box::new(Error::Ractor(e)) as Box<dyn std::error::Error + Send + Sync>)
            },
            Msg::StartListen => {
                let mut out = stdout();
                enable_raw_mode()?;
                execute!(&mut out, EnterAlternateScreen)?;
                cast!(myself, Msg::Read)
                    .map_err(|e| Box::new(Error::Ractor(e)) as Box<dyn std::error::Error + Send + Sync>)
            }
            Msg::StopListen => {
                let mut out = stdout();
                execute!(&mut out, LeaveAlternateScreen)?;
                disable_raw_mode()
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Msg::Event(e) => Err(Box::new(Error::LoopedBack(e))),
        }
    }
}
