use std::thread;

const PHILOSOPHERS_COUNT: u8 = 5;


fn philosopher() {

}


fn main() {
    let mut threads = vec![];

    for i in 0..PHILOSOPHERS_COUNT {
        threads.push(thread::spawn(move || philosopher()));
    }

    // Wait for threads to finish
    for thread in threads {
        thread.join().expect("Thread panicked!");
    }
}
