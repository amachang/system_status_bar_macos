use std::{
    sync::mpsc::channel,
    thread::*,
    time::*,
    cell::*,
};
use system_status_bar_macos::*;
use sysinfo::*;

fn main() {
    let (sender, receiver) = channel::<()>();

    // thread that sends command to event loop
    spawn(move || {
        loop {
            sender.send(()).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    let status_item = RefCell::new(StatusItem::new("", Menu::new(vec![])));

    sync_infinite_event_loop(receiver, move |_| {
        let mut sys = System::new_all();
        sys.refresh_all();

        status_item.borrow_mut().set_title(format!("CPU Usage: {:3.2}%", sys.global_cpu_info().cpu_usage()));
    });
}


