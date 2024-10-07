use std::{future::Future, time::Duration};

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

async fn select<A: Future, B: Future>(left: A, right: B) -> Either<A::Output, B::Output> {
    // REPLACE ME
    tokio::select! {
        left = left => Either::Left(left),
        right = right => Either::Right(right),
    }
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

    let res = select(left, right).await;

    println!("raced: {:?}", res);
}
