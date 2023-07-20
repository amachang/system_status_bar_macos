use std::{
    sync::mpsc::channel,
    thread::*,
    time::*,
};
use system_status_bar_macos::*;

fn main() {
    let _status_item = StatusItem::new("EXAMPLE", Menu::new(vec![]));
    let (_sender, receiver) = channel::<()>();
    let (event_loop, terminator) = sync_event_loop(receiver, |_| { });

    spawn(move || {
        sleep(Duration::from_secs(10));

        terminator.terminate(); // do it
    });

    event_loop();
}
