use std::{
    time::{
        Duration,
    },
    cell::{
        RefCell,
    },
};

use system_status_bar_macos::{
    StatusItem,
    Menu,
    MenuItem,
    async_event_loop,
};

use tokio::{
    time::{
        sleep,
    },
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (event_loop, terminator) = async_event_loop(sleep);
    let event_loop = tokio::spawn(event_loop);

    let status_item = RefCell::new(StatusItem::new("EXAMPLE", Menu::new(vec![])));

    for loop_count in 0..5 {
        status_item.borrow_mut().set_menu(Menu::new(vec![
                MenuItem::new(format!("Count: {}", loop_count), None, Some(Menu::new(vec![
                        MenuItem::new("Sub menu", None, None),
                ]))),
                MenuItem::new(format!("Count: {}", loop_count), Some(Box::new(|_| {
                    println!("Clicked");
                })), None),
        ]));
        sleep(Duration::from_secs(1)).await;
    }

    terminator.terminate();
    event_loop.await.unwrap();
}

