use std::sync::mpsc::channel;
use system_status_bar_macos::*;

fn main() {
    let _status_item = StatusItem::new("HELLO_WORLD", Menu::new(vec![]));

    let (_sender, receiver) = channel::<()>();
    sync_infinite_event_loop(receiver, |_| { });
}

