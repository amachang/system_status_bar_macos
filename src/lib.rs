use std::{
    time::{
        Duration,
    },
    thread::{
        sleep,
    },
    future::{
        Future,
    },
    sync::{
        mpsc::{
            channel,
            Sender,
            Receiver,
            TryRecvError,
        },
    },
    ffi::{
        c_void,
    },
};

use objc2::{
    rc::{
        Id,
    },
    ffi::{
        objc_autoreleasePoolPush,
        objc_autoreleasePoolPop,
    },
};

use icrate::{
    Foundation::{
        NSString,
    },
    AppKit::{
        NSEvent,
        NSStatusBar,
        NSStatusItem,
        NSMenu,
        NSMenuItem,
        NSApplication,
        NSEventMaskAny,
        NSVariableStatusItemLength,
    },
};

#[derive(Debug)]
pub struct StatusItem {
    inner: Id<NSStatusItem>,

    menu: Menu,
    title: String,
}

impl StatusItem {
    pub fn new(title: impl AsRef<str>, menu: Menu) -> Self {
        let title = title.as_ref();
        unsafe {
            // initialize if not yet
            NSApplication::sharedApplication();

            let bar = NSStatusBar::systemStatusBar();
            let inner = bar.statusItemWithLength(NSVariableStatusItemLength);

            inner.setMenu(Some(&menu.inner));
            inner.button().unwrap().setTitle(&NSString::from_str(title));

            let title = title.to_string();
            Self { inner, menu, title }
        }
    }

    pub fn menu(&self) -> &Menu {
        &self.menu
    }

    pub fn set_menu(&mut self, menu: Menu) {
        unsafe {
            self.inner.setMenu(Some(&menu.inner));
            self.menu = menu;
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: impl AsRef<str>) {
        let title = title.as_ref();
        unsafe {
            self.inner.button().unwrap().setTitle(&NSString::from_str(title));
            self.title = title.to_string();
        }
    }
}

impl Drop for StatusItem {
    fn drop(&mut self) {
        unsafe {
            self.inner.setMenu(None);

            let bar = NSStatusBar::systemStatusBar();
            bar.removeStatusItem(&self.inner);
        }
    }
}


#[derive(Debug)]
pub struct Menu {
    inner: Id<NSMenu>,

    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        unsafe {
            let inner = NSMenu::new();

            for item in &items {
                inner.addItem(&item.inner);
            }

            Self { inner, items }
        }
    }

    pub fn items(&self) -> &Vec<MenuItem> {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<MenuItem>) {
        unsafe {
            self.inner.removeAllItems();
            for item in &items {
                self.inner.addItem(&item.inner);
            }
            self.items = items;
        }
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unsafe {
            self.inner.removeAllItems();
            self.items = vec![];
        }
    }
}

#[derive(Debug)]
pub struct MenuItem {
    inner: Id<NSMenuItem>,
}

impl MenuItem {
    pub fn new(_: impl AsRef<str>, _: Option<Box<dyn Fn(&MenuItem)>>, _: Option<Menu>) -> Self {
        todo!();
    }
}

#[derive(Debug)]
pub struct LoopTerminator {
    sender: Sender<()>,
}

impl LoopTerminator {
    pub fn new() -> (Self, LoopTerminatee) {
        let (sender, receiver) = channel::<()>();
        (Self { sender }, LoopTerminatee { receiver })
    }

    pub fn terminate(&self) {
        self.sender.send(()).unwrap();
    }
}

#[derive(Debug)]
pub struct LoopTerminatee {
    receiver: Receiver<()>,
}

impl LoopTerminatee {
    pub fn should_terminate(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(()) => true,
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => true,
        }
    }
}

struct AutoReleasePoolContext(*mut c_void);
unsafe impl Send for AutoReleasePoolContext { }

macro_rules! event_loop {
    ($terminatee: expr, $sleep: expr, $receiver_callback: expr) => {
        unsafe {
            let run_mode = NSString::from_str("kCFRunLoopDefaultMode");
            {
                let app = NSApplication::sharedApplication();
                app.finishLaunching();
            }
            'event_loop: loop {
                let pool_ctx = AutoReleasePoolContext(objc_autoreleasePoolPush());
                for _ in 0..100 {
                    {
                        let app = NSApplication::sharedApplication();
                        if $terminatee.should_terminate() {
                            break 'event_loop;
                        }

                        $receiver_callback;

                        let event: Option<Id<NSEvent>> = app.nextEventMatchingMask_untilDate_inMode_dequeue(NSEventMaskAny, None, &run_mode, true);
                        if let Some(event) = event {
                            app.sendEvent(&event);
                        };
                        app.updateWindows();
                    }
                    $sleep;
                }
                objc_autoreleasePoolPop(pool_ctx.0);
            }
        };
    }
}

pub fn sync_event_loop<T>(receiver: Receiver<T>, callback: impl Fn(T)) -> (impl Fn(), LoopTerminator) {
    let (terminator, terminatee) = LoopTerminator::new();
    let f = move || {
        event_loop!(terminatee, sleep(Duration::from_millis(10)), if let Ok(data) = receiver.try_recv() { callback(data) });
    };
    (f, terminator)
}

pub fn async_event_loop<F>(async_sleep: impl Fn(Duration) -> F) -> (impl Future<Output = ()> , LoopTerminator)
where
    F: Future<Output = ()>,
{
    let (terminator, terminatee) = LoopTerminator::new();
    let future = async move {
        event_loop!(terminatee, async_sleep(Duration::from_millis(10)).await, ());
    };
    (future, terminator)
}

