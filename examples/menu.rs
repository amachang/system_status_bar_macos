use std::sync::mpsc::channel;
use system_status_bar_macos::*;

fn main() {
    let mut menu_with_image = MenuItem::new(
        "MENU WITH IMAGE",
        Some(Box::new(|| {
            println!("yummy!");
        })),
        None,
    );
    if let Some(coffee_image) = Image::with_system_symbol_name("mug.fill", Some("Coffee!")) {
        menu_with_image.set_image(coffee_image);
    }

    let mut menu_with_control_state = MenuItem::new(
        "MENU WITH CONTROL STATE",
        Some(Box::new(|| {
            println!("toggle control state?");
        })),
        None,
    );
    menu_with_control_state.set_control_state(ControlState::On);

    let _status_item = StatusItem::new(
        "TITLE",
        Menu::new(vec![
            MenuItem::new("UNCLICKABLE MENU", None, None),
            MenuItem::new(
                "CLICKABLE MENU",
                Some(Box::new(|| {
                    println!("clicked!");
                })),
                None,
            ),
            menu_with_image,
            menu_with_control_state,
            MenuItem::separator(),
            MenuItem::new(
                "PARENT MENU",
                None,
                Some(Menu::new(vec![
                    MenuItem::new("SUBMENU", None, None),
                    MenuItem::new("SUBMENU", None, None),
                ])),
            ),
        ]),
    );

    let (_sender, receiver) = channel::<()>();
    sync_infinite_event_loop(receiver, |_| {});
}
