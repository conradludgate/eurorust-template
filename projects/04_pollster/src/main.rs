use std::future::Future;

pub fn block_on<F: Future>(_f: F) -> F::Output {
    todo!()
}

fn main() {
    let (tx, rx) = tokio::sync::oneshot::channel();

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        tx.send("hello world").unwrap();
    });

    let value = block_on(async { rx.await.unwrap() });

    println!("{value}");
}
