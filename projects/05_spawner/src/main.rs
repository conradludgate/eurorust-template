use std::future::Future;

pub fn block_on<F: Future>(_f: F) -> F::Output {
    todo!()
}

pub fn spawn<F: Future<Output = ()> + Send + 'static>(_f: F) {
    todo!()
}

fn main() {
    block_on(async move {
        let (watch_tx, watch_rx) = tokio::sync::watch::channel(true);

        for i in 0..10 {
            let mut watch_rx = watch_rx.clone();
            spawn(async move {
                // wait until we are no longer running
                watch_rx.wait_for(|running| !*running).await.unwrap();
                println!("completed {i}")
            });
        }

        let (tx, rx) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            watch_tx.send(false).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(2));
            tx.send(()).unwrap();
        });

        rx.await.unwrap();
    });

    println!("done");
}
