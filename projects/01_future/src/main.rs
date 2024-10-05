use std::time::Duration;

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = tx.send(());
    });

    let left = tokio::time::sleep(Duration::from_secs(3));
    let right = rx;

    // let res = Select { left, right }.await;

    let res = tokio::select! {
        left = left => Either::Left(left),
        right = right => Either::Right(right),
    };

    println!("raced: {:?}", res);
}
