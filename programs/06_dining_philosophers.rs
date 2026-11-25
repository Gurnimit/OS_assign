use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const N: usize = 5;

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
    let forks: Arc<Vec<Semaphore>> = Arc::new((0..N).map(|_| Semaphore::new(1)).collect());
    let room = Arc::new(Semaphore::new(N - 1));
    let mut handles = Vec::with_capacity(N);

    for id in 0..N {
        let forks = Arc::clone(&forks);
        let room = Arc::clone(&room);

        handles.push(thread::spawn(move || {
            let left = id;
            let right = (id + 1) % N;

            for _cycle in 0..2 {
                println!("Philosopher {} is thinking", id);
                thread::sleep(Duration::from_millis(100));

                room.wait();
                forks[left].wait();
                forks[right].wait();

                println!(
                    "Philosopher {} is eating (fork {} and fork {})",
                    id, left, right
                );
                thread::sleep(Duration::from_millis(150));

                forks[right].post();
                forks[left].post();
                room.post();

                println!("Philosopher {} finished eating and put down forks", id);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
