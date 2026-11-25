use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Semaphore {
    count: Mutex<usize>,
    cvar: Condvar,
}

impl Semaphore {
    fn new(initial: usize) -> Self {
        Semaphore {
            count: Mutex::new(initial),
            cvar: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn post(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.cvar.notify_one();
    }
}

fn main() {
    let mutex = Arc::new(Semaphore::new(1));
    let wrt = Arc::new(Semaphore::new(1));
    let read_count = Arc::new(AtomicUsize::new(0));
    let shared_data = Arc::new(AtomicI32::new(0));

    let mut handles = Vec::new();

    {
        let wrt = Arc::clone(&wrt);
        let shared_data = Arc::clone(&shared_data);
        handles.push(thread::spawn(move || {
            wrt.wait();
            let val = shared_data.fetch_add(1, Ordering::SeqCst) + 1;
            println!("Writer 1: wrote shared_data = {}", val);
            thread::sleep(Duration::from_millis(300));
            wrt.post();
        }));
    }

    for id in 1..=5 {
        let mutex = Arc::clone(&mutex);
        let wrt = Arc::clone(&wrt);
        let read_count = Arc::clone(&read_count);
        let shared_data = Arc::clone(&shared_data);
        handles.push(thread::spawn(move || {
            mutex.wait();
            if read_count.fetch_add(1, Ordering::SeqCst) + 1 == 1 {
                wrt.wait();
            }
            mutex.post();

            let val = shared_data.load(Ordering::SeqCst);
            println!("Reader {}: read shared_data = {}", id, val);
            thread::sleep(Duration::from_millis(200));

            mutex.wait();
            if read_count.fetch_sub(1, Ordering::SeqCst) - 1 == 0 {
                wrt.post();
            }
            mutex.post();
        }));
    }

    {
        let wrt = Arc::clone(&wrt);
        let shared_data = Arc::clone(&shared_data);
        handles.push(thread::spawn(move || {
            wrt.wait();
            let val = shared_data.fetch_add(1, Ordering::SeqCst) + 1;
            println!("Writer 2: wrote shared_data = {}", val);
            thread::sleep(Duration::from_millis(300));
            wrt.post();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
