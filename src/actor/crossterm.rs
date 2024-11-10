use std::io::stdout;
use crossterm::{
    execute,
    event::{
        EventStream, KeyEvent, KeyModifiers,
        KeyCode::Char, Event::Key,
    },
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use kameo::{
    Actor, Reply,
    actor::{ActorRef, WeakActorRef},
    request::MessageSend,
    error::{BoxError, ActorStopReason, PanicError},
    message::{Context, Message},
    mailbox::bounded::BoundedMailbox,
};
use futures_util::StreamExt;

#[derive(Reply)]
pub struct Event(crossterm::event::Event);

pub struct CrossTerm {
    stream: EventStream,
}

impl CrossTerm {
    pub fn new() -> Self {
        CrossTerm {
            stream: EventStream::new(),
        }
    }

    // TODO add return type for error handling
    fn exit(&self) {
        let mut out = stdout();
        execute!(&mut out, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}

impl Actor for CrossTerm {
    type Mailbox = BoundedMailbox<Self>;

    fn name() -> &'static str {
        "CrossTerm"
    }

    async fn on_start(&mut self, actor_ref: ActorRef<Self>) -> Result<(), BoxError> {
        let mut out = stdout();
        enable_raw_mode().unwrap();
        execute!(&mut out, EnterAlternateScreen).unwrap();
        actor_ref.tell(Read()).send().await.unwrap();
        Ok(())
        
    }

    async fn on_panic(&mut self, _actor_ref: WeakActorRef<Self>, _reason: PanicError) -> Result<Option<ActorStopReason>, BoxError> {
        self.exit();
        Ok(None)
    }

    async fn on_stop(self, actor_ref: WeakActorRef<Self>, _reason: ActorStopReason) -> Result<(), BoxError> {
        self.exit();
        Ok(())
    }
}

pub struct Read();

impl Message<Read> for CrossTerm {
    type Reply = Event;

    async fn handle(&mut self, msg: Read, ctx: Context<'_, Self, Self::Reply>) -> Self::Reply {
        let event = self.stream.next().await.unwrap().unwrap();
        if let Key(KeyEvent {code: Char('c'), modifiers: KeyModifiers::CONTROL, kind: _p, state: _s}) = event {
            ctx.actor_ref().kill();
        }

        ctx.actor_ref().tell(Read()).send().await.unwrap();
        println!("{event:?}");
        Event(event)
    }
}
