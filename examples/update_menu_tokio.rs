use system_status_bar_macos::*;
use sysinfo::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    spawn(async_infinite_event_loop(time::sleep));

    let mut status_item = StatusItem::new("", Menu::new(vec![]));
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        status_item.set_title(format!("CPU Usage: {:3.2}%", sys.global_cpu_info().cpu_usage()));
        status_item.set_menu(Menu::new(vec![
            MenuItem::new(format!("Used {} bytes memory", sys.used_memory()), None, None),
            MenuItem::new(format!("Used {} bytes swap", sys.used_swap()), None, None),
        ]));
        time::sleep(time::Duration::from_secs(1)).await;
    }
}

