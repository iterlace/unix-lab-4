use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use rand::Rng;

const PHILOSOPHERS_COUNT: u8 = 5;
const THINKING_DURATION: u8 = 4;  // in seconds
const EATING_DURATION: u8 = 5;

//
//      o2  —3—  o3
//    —2—          —4—
//  o1                o4
//      —1—      —5—
//           o5
//
// Where oN is a philosopher N, —N— is a fork N.
// To eat, a philosopher must acquire —N— and —(N+1)—
// When philosopher intends to eat, they acquire a global control lock.

fn left_fork(i: u8) -> u8 {
    return i;
}

fn right_fork(i: u8) -> u8 {
    return (i+1) % PHILOSOPHERS_COUNT;
}

fn log(msg: &str, philosopher_id: u8) {
    let whitespaces = " ".repeat((philosopher_id * 4) as usize);
    println!("{}{} {}", whitespaces, philosopher_id, msg);
}

fn take_forks_and_eat(philosopher_id: u8, control_mutex: Arc<Mutex<()>>, fork_mutexes: Arc<Vec<Mutex<()>>>) {
    let left = &fork_mutexes.clone()[left_fork(philosopher_id) as usize];
    let right = &fork_mutexes.clone()[right_fork(philosopher_id) as usize];

    let mut left_mutex_guard: Option<MutexGuard<()>> = None;
    let mut right_mutex_guard: Option<MutexGuard<()>> = None;

    {
        // wait for control_mutex, and then acquire it
        log("is waiting for control_mutex", philosopher_id);
        let control_mutex_guard = control_mutex.lock().unwrap();
        log("has acquired a control_mutex", philosopher_id);

        while left_mutex_guard.is_none() && right_mutex_guard.is_none() {
            left.try_lock()
                .and_then(|lmg| right.try_lock()
                    .and_then(|rmg| {
                        left_mutex_guard = Some(lmg);
                        right_mutex_guard = Some(rmg);
                        log("has acquired both forks", philosopher_id);
                        Ok(())
                    })
                )
                .unwrap_or_else(|_| ());
        }
        log("has released a control_mutex", philosopher_id);
    }

    log("has started eating their meal", philosopher_id);
    thread::sleep(Duration::from_secs(EATING_DURATION as u64));
    log("has finished eating their meal", philosopher_id);
}

fn think(philosopher_id: u8) -> () {
    log(format!("is thinking for {} seconds...", THINKING_DURATION).as_str(), philosopher_id);
    thread::sleep(Duration::from_secs(THINKING_DURATION as u64));
}

fn philosopher(philosopher_id: u8, control_mutex: Arc<Mutex<()>>, fork_mutexes: Arc<Vec<Mutex<()>>>) {
    loop {
        think(philosopher_id.clone());
        take_forks_and_eat(philosopher_id, control_mutex.clone(), fork_mutexes.clone());
    }
}


fn main() {
    let control_mutex = Arc::new(Mutex::new(()));
    let mut fork_mutexes = Arc::new(
        (0..PHILOSOPHERS_COUNT)
            .map(|_| Mutex::new(()))
            .collect::<Vec<_>>()
    );

    let mut threads = vec![];

    for i in 0..PHILOSOPHERS_COUNT {
        let control_mutex_clone = control_mutex.clone();
        let fork_mutexes_clone = fork_mutexes.clone();
        threads.push(
            thread::Builder::new()
                .name(format!("Philosopher #{}", &i))
                .spawn(move || philosopher(
                    i,
                    control_mutex_clone,
                    fork_mutexes_clone
                ))
                .unwrap()
        );
    }

    // Wait for threads to finish
    for thread in threads {
        thread.join().expect("Thread panicked!");
    }
    thread::sleep(Duration::from_secs(20));
}
