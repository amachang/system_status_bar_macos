use std::{
    time::{
        Duration,
    },
};

use system_status_bar_macos::{
    Error,
    LoopTerminator,
    async_event_loop,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let (terminator, terminatee) = LoopTerminator::new();

    tokio::spawn(async {
        async_event_loop(terminatee, tokio::time::sleep).await
    });

    tokio::time::sleep(Duration::from_secs(3)).await;
    terminator.terminate()?;

    Ok(())
}

