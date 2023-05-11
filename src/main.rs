use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

const PHILOSOPHERS_COUNT: u8 = 5;
const THINKING_DURATION: u8 = 5;  // in seconds

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


fn take_forks(philosopher_id: u8, control_mutex: Arc<Mutex<()>>, fork_mutexes: Arc<Vec<Mutex<()>>>) {
    let control_mutex_guard = control_mutex.lock().unwrap();
    println!("{:?} has acquired a control_mutex", thread::current().name());
}

fn put_forks_back(philosopher_id: u8, control_mutex: Arc<Mutex<()>>, fork_mutexes: Arc<Vec<Mutex<()>>>) {

}

fn think(philosopher_id: u8) -> () {
    println!("{} is thinking for {} seconds...", thread::current().name().unwrap(), THINKING_DURATION);
    thread::sleep(Duration::from_secs(THINKING_DURATION as u64));
}

fn philosopher(philosopher_id: u8, control_mutex: Arc<Mutex<()>>, fork_mutexes: Arc<Vec<Mutex<()>>>) {
    loop {
        think(philosopher_id.clone());
        take_forks(philosopher_id, control_mutex.clone(), fork_mutexes.clone());
        eat();

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
                    1,
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
