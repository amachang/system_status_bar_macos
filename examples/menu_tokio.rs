use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let event_loop = spawn(async_infinite_event_loop(time::sleep));

    let _status_item = StatusItem::new("TITLE", Menu::new(vec![
            MenuItem::new("UNCLICKABLE MENU", None, None),
            MenuItem::new("CLICKABLE MENU", Some(Box::new(|| {
                println!("clicked!");
            })), None),
            MenuItem::new("PARENT MENU", None, Some(Menu::new(vec![
                MenuItem::new("SUBMENU", None, None),
                MenuItem::new("SUBMENU", None, None),
            ]))),
    ]));

    event_loop.await.unwrap();
}

