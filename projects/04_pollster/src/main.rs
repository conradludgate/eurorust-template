use std::{future::Future, task::Poll, time::Instant};

pub fn block_on<F: Future>(_f: F) -> F::Output {
    todo!()
}

fn main() {
    let start = std::time::Instant::now();
    let deadline = start + std::time::Duration::from_secs(1);
    let woken = block_on(async { bad_sleep(deadline).await });

    let lag = woken - deadline;

    println!("{lag:?}");
}

/// Sleep until the deadline, returning the time we actually woke up.
async fn bad_sleep(deadline: Instant) -> Instant {
    std::future::poll_fn(|cx| {
        let now = std::time::Instant::now();
        if deadline < now {
            return Poll::Ready(now);
        }

        let waker = cx.waker().clone();

        // spawn a new thread to run the sleep.
        std::thread::spawn(move || {
            std::thread::sleep(deadline - now);
            // wake up the task after the sleep
            waker.wake();
        });

        Poll::Pending
    })
    .await
}
