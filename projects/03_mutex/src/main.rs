use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};

struct AsyncMutex<T> {
    data: UnsafeCell<T>,
}

struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
}

// Safety: We are a mutex :)
unsafe impl<T: Send> Send for AsyncMutex<T> {}
unsafe impl<T: Send> Sync for AsyncMutex<T> {}
unsafe impl<T: Sync + Send> Sync for AsyncMutexGuard<'_, T> {}

impl<T> Deref for AsyncMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}
impl<T> DerefMut for AsyncMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let mutex = Arc::new(AsyncMutex::new(0));
    let mutex = Arc::new(tokio::sync::Mutex::new(0));

    let mutex1 = mutex.clone();
    tokio::spawn(async move {
        println!("task 1 acquiring the lock");
        let mut lock = mutex1.lock().await;
        println!("task 1 acquired the lock");

        tokio::time::sleep(Duration::from_millis(2000)).await;

        *lock += 1;
        println!("task 1 releasing the lock");
    });

    let mutex2 = mutex.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1000)).await;

        println!("task 2 acquiring the lock");
        let mut lock = mutex2.lock().await;
        println!("task 2 acquired the lock");

        tokio::time::sleep(Duration::from_millis(2000)).await;

        *lock += 1;
        println!("task 2 releasing the lock");
    });

    tokio::time::sleep(Duration::from_millis(1500)).await;

    println!("task 0 acquiring the lock");
    let val = *mutex.lock().await;
    println!("task 0 acquired the lock");

    println!("Shutting down with value {val}");
}
