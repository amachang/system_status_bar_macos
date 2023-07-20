use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (event_loop, terminator) = async_event_loop(time::sleep);
    let event_loop = spawn(event_loop);

    let _status_item = StatusItem::new("EXAMPLE", Menu::new(vec![]));
    time::sleep(time::Duration::from_secs(10)).await;

    terminator.terminate();
    event_loop.await.unwrap();
}

