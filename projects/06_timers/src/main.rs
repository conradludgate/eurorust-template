use std::{future::Future, time::Instant};

pub fn block_on<F: Future>(_f: F) -> F::Output {
    todo!()
}

pub fn spawn<F: Future<Output = ()> + Send + 'static>(_f: F) {
    todo!()
}

/// Sleep until the deadline, returning the time we actually woke up.
pub async fn sleep_until(_deadline: Instant) -> Instant {
    todo!()
}

fn main() {
    let start = std::time::Instant::now();

    let _woken = block_on(async move {
        for i in 0..10 {
            spawn(async move {
                sleep_until(start + std::time::Duration::from_secs(1)).await;
                println!("completed {i}")
            });
        }

        sleep_until(start + std::time::Duration::from_secs(2)).await
    });

    println!("done");
}
