use tokio::sync::Notify;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

/// The following is an easier implementation of the future-delay example
/// that uses instead `tokio::sync::Notify`.
///
/// It still spawns a new thread for every call, though, so yes this is
/// just an excuse to show how to use `Notify` in action.
async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });


    notify.notified().await;
}

#[tokio::main]
async fn main() {
    delay(Duration::from_millis(1000)).await;
}