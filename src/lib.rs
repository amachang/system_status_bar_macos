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
        NSApplication,
        NSEventMaskAny,
    },
};

#[derive(Debug)]
pub struct StatusItem {
}

impl StatusItem {
    pub fn new(_: impl AsRef<str>, _: Menu) -> Self {
        todo!();
    }

    pub fn set_menu(&mut self, _: Menu) {
        todo!();
    }
}

#[derive(Debug)]
pub struct Menu {
}

impl Menu {
    pub fn new(_: Vec<MenuItem>) -> Self {
        todo!();
    }
}

#[derive(Debug)]
pub struct MenuItem {
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

