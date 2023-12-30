use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub(super) fn channel_demo() -> () {
    // Like go, Rust has channels to allow passing messages between threads
    // This allows threads to function like actors, so a similar concept to Akka too

    // Multi-producer, single consumer. Like an akka fan-in.
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for i in 1..20 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for i in 20..40 {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(80));
        }
    });

    // This can be simplified: `for msg in rx` would work as rx is an iterator
    for _ in 1..100 {
        thread::sleep(Duration::from_millis(100));

        // ... but this is a good opportunity to demo the if let syntax, which is fairly clear
        if let Some(msg) = rx.try_recv().ok() {
            println!("Got message: {}", msg);

            if msg == 10 {
                println!("Terminating on message {}", msg);
                break;
            }
        }
    }
}
