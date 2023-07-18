use std::{
    time::{
        Duration,
    },
    future::{
        Future,
    },
    sync::{
        mpsc::{
            channel,
            Sender,
            Receiver,
            SendError,
            TryRecvError,
        },
    },
};

#[derive(Debug)]
pub enum Error {
    MessageSendError(SendError<()>)
}

impl From<SendError<()>> for Error {
    fn from(err: SendError<()>) -> Self {
        Self::MessageSendError(err)
    }
}

#[derive(Debug)]
pub struct StatusBar {
}

impl StatusBar {
    pub fn system() -> Self {
        todo!();
    }

    pub fn new_item(&self) -> &StatusItem {
        todo!();
    }

    pub fn remove_item(&mut self, _: &StatusItem) {
        todo!();
    }
}

#[derive(Debug)]
pub struct StatusItem {
}

impl StatusItem {
    pub fn bar(&self) -> Option<&StatusBar> {
        todo!();
    }

    pub fn menu(&self) -> Option<&Menu> {
        todo!();
    }

    pub fn set_menu(&mut self, _: Menu) {
        todo!();
    }

    pub fn title(&self) -> &str {
        todo!();
    }

    pub fn set_title(&mut self, _: impl AsRef<str>) {
        todo!();
    }

}

#[derive(Debug)]
pub struct Menu {
}

impl Menu {
    pub fn new() -> Self {
        todo!();
    }

    pub fn item(&self, _: usize) -> &MenuItem {
        todo!();
    }

    pub fn items(&self) -> Vec<&MenuItem> {
        todo!();
    }

    pub fn set_items(&mut self, _: Vec<MenuItem>) {
        todo!();
    }

    pub fn insert_item(&mut self, _: MenuItem, _: usize) {
        todo!();
    }

    pub fn add_item(&mut self, _: MenuItem) {
        todo!();
    }

    pub fn remove_item(&mut self, _: MenuItem) {
        todo!();
    }

    pub fn remove_item_at(&mut self, _: usize) {
        todo!();
    }

    pub fn remove_all_items(&mut self) {
        todo!();
    }

    pub fn number_of_items(&self) -> usize {
        todo!();
    }
}

#[derive(Debug)]
pub struct MenuItem {
}

impl MenuItem {
    pub fn new<S, F>(_: S, _: Option<F>)
        where
            S: AsRef<str>,
            F: Fn(&MenuItem),
    {
        todo!();
    }

    pub fn title(&self) -> &str {
        todo!();
    }
}

#[derive(Debug)]
pub struct LoopTerminator {
    sender: Sender<()>,
}

impl LoopTerminator {
    pub fn new() -> (Self, LoopTerminatee) {
        let (sender, receiver) = channel::<()>();
        (Self { sender }, LoopTerminatee { receiver })
    }

    pub fn terminate(&self) -> Result<(), Error> {
        self.sender.send(())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LoopTerminatee {
    receiver: Receiver<()>,
}

impl LoopTerminatee {
    pub fn should_terminate(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(()) => true,
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => true,
        }
    }
}

pub fn start_event_loop() -> LoopTerminator {
    todo!();
}

pub async fn async_event_loop<F>(_: LoopTerminatee, _: impl Fn(Duration) -> F)
where
    F: Future<Output = ()>
{
    todo!();
}

