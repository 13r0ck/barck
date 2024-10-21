use ractor::{Actor, ActorRef, ActorProcessingErr};

pub struct Ratatui;
pub enum Msg {}

#[async_trait::async_trait]
impl Actor for Ratatui {
    type State = ();
    type Msg = Msg;
    type Arguments = ();

    async fn pre_start(&self, _myself: ActorRef<Self::Msg>, _arguments: Self::Arguments) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
        }
    }
}
