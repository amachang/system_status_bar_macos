use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _status_item = StatusItem::new("HELLO_WORLD", Menu::new(vec![]));

    spawn(async_infinite_event_loop(time::sleep)).await.unwrap();
}

