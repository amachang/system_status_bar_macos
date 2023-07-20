use std::{
    time::{
        Duration,
    },
    thread::{
        self,
        sleep,
    },
    sync::{
        mpsc::{
            channel,
        },
    },
    cell::{
        RefCell,
    },
};

use system_status_bar_macos::{
    StatusItem,
    Menu,
    MenuItem,
    sync_event_loop,
};

enum Command {
    Update(usize),
}

fn main() {
    let (sender, receiver) = channel::<Command>();
    let status_item = RefCell::new(StatusItem::new("EXAMPLE", Menu::new(vec![])));

    let status_item = &status_item;
    let (event_loop, terminator) = sync_event_loop(receiver, move |command| {
        match command {
            Command::Update(loop_count) => {
                status_item.borrow_mut().set_menu(Menu::new(vec![
                        MenuItem::new(format!("Count: {}", loop_count), None, Some(Menu::new(vec![
                                MenuItem::new("Sub menu", None, None),
                        ]))),
                        MenuItem::new(format!("Count: {}", loop_count), Some(Box::new(|| {
                            println!("Clicked");
                        })), None),
                ]));
            },
        };
    });

    thread::spawn(move || {
        for loop_count in 0..5 {
            sender.send(Command::Update(loop_count)).unwrap();
            sleep(Duration::from_secs(1));
        }
        terminator.terminate();
    });

    event_loop();
}


