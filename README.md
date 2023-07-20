# system\_status\_bar\_macos

Library for interacting with the system's status bar for macOS, or more simply, the one for using `[NSStatusBar systemStatusBar]`.

<img width="318" alt="screenshot" src="https://github.com/amachang/system_status_bar_macos/assets/10735/b3e2787b-77fe-4dd5-a560-6633042d5066">

## Example 1: Hello, World!

``` rust
use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _status_item = StatusItem::new("HELLO_WORLD", Menu::new(vec![]));

    spawn(async_infinite_event_loop(time::sleep)).await.unwrap();
}
```

without async runtime:

```rust
use std::sync::mpsc::channel;
use system_status_bar_macos::*;

fn main() {
    let _status_item = StatusItem::new("HELLO_WORLD", Menu::new(vec![]));

    let (_sender, receiver) = channel::<()>();
    sync_infinite_event_loop(receiver, |_| { });
}
```

## Example 2: Show CPU usage on the status bar

```rust
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
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
```

without async runtime:

```rust
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
```

## Example 3: Show menus (clickable, unclickable, and having submenus)

```rust
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
```

without async runtime:

```rust
use std::sync::mpsc::channel;
use system_status_bar_macos::*;

fn main() {
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

    let (_sender, receiver) = channel::<()>();
    sync_infinite_event_loop(receiver, |_| { });
}
```

## Example 4: Update menus

```rust
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
```

without async runtime

```rust
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
        status_item.borrow_mut().set_menu(Menu::new(vec![
            MenuItem::new(format!("Used {} bytes memory", sys.used_memory()), None, None),
            MenuItem::new(format!("Used {} bytes swap", sys.used_swap()), None, None),
        ]));
    });
}
```

## Example 5: Break event loop

```rust
use system_status_bar_macos::*;
use tokio::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (event_loop, terminator) = async_event_loop(time::sleep);
    let event_loop = spawn(event_loop);

    let _status_item = StatusItem::new("EXAMPLE", Menu::new(vec![]));
    time::sleep(time::Duration::from_secs(10)).await;

    terminator.terminate(); // break event loop

    event_loop.await.unwrap();
}
```

without async runtime

```rust
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

        terminator.terminate(); // break event loop
    });

    event_loop();
}
```


License: MIT OR Apache-2.0
