use std::{thread, time::Duration};

fn main() {
    let threads: Vec<_> = (0..500)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread #{i} started!");
                thread::sleep(Duration::from_millis(500));
                println!("Thread #{i} finished!");
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }
}
