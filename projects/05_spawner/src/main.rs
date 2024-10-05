use std::{future::Future, task::Poll, time::Instant};

pub fn block_on<F: Future>(_f: F) -> F::Output {
    todo!()
}

pub fn spawn<F: Future<Output = ()> + Send + 'static>(_f: F) {
    todo!()
}

fn main() {
    let start = std::time::Instant::now();

    let _woken = block_on(async move {
        for i in 0..10 {
            spawn(async move {
                bad_sleep(start + std::time::Duration::from_secs(1)).await;
                println!("completed {i}")
            });
        }

        bad_sleep(start + std::time::Duration::from_secs(2)).await
    });

    println!("done");
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
