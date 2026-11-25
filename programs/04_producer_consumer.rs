use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const BUFFER_SIZE: usize = 5;
const ITEMS: usize = 10;

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

struct SharedBuffer {
    buffer: [usize; BUFFER_SIZE],
    in_idx: usize,
    out_idx: usize,
}

fn main() {
    let empty = Arc::new(Semaphore::new(BUFFER_SIZE));
    let full = Arc::new(Semaphore::new(0));
    let mutex = Arc::new(Semaphore::new(1));
    let shared = Arc::new(Mutex::new(SharedBuffer {
        buffer: [0; BUFFER_SIZE],
        in_idx: 0,
        out_idx: 0,
    }));

    let prod_handle = {
        let empty = Arc::clone(&empty);
        let full = Arc::clone(&full);
        let mutex = Arc::clone(&mutex);
        let shared = Arc::clone(&shared);

        thread::spawn(move || {
            for item in 1..=ITEMS {
                empty.wait();
                mutex.wait();

                {
                    let mut state = shared.lock().unwrap();
                    let slot = state.in_idx;
                    state.buffer[slot] = item;
                    println!("Producer produced item {} at slot {}", item, slot);
                    state.in_idx = (slot + 1) % BUFFER_SIZE;
                }

                mutex.post();
                full.post();
                thread::sleep(Duration::from_millis(100));
            }
        })
    };

    let cons_handle = {
        let empty = Arc::clone(&empty);
        let full = Arc::clone(&full);
        let mutex = Arc::clone(&mutex);
        let shared = Arc::clone(&shared);

        thread::spawn(move || {
            for _ in 1..=ITEMS {
                full.wait();
                mutex.wait();

                {
                    let mut state = shared.lock().unwrap();
                    let slot = state.out_idx;
                    let item = state.buffer[slot];
                    println!("\t\tConsumer consumed item {} from slot {}", item, slot);
                    state.out_idx = (slot + 1) % BUFFER_SIZE;
                }

                mutex.post();
                empty.post();
                thread::sleep(Duration::from_millis(150));
            }
        })
    };

    prod_handle.join().unwrap();
    cons_handle.join().unwrap();
    println!("All items produced and consumed.");
}
