use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn concurrent() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    for i in 0..10 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            // process written below is an experiment to do something which takes long time in some threads
            if i%3 == 0 {
                thread::sleep(Duration::from_millis(100));
            }

            data[i] += 1;
        });
    }

    // In this type (written above) of concurrent process,
    // `data`'s elements are locked by Mutex trait method.
    // Therefore, if you try to get value of some of them before thread closes,
    // you will get locked value (and you will fail processes after that getting value)
    // Because of this reason, you should wait some time before you try to get value.

    // thread::sleep(Duration::from_millis(3));

    println!("{:?}", data);
    // when you wait enough
    //   #=> [2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    // when you do not wait enough
    //   #=> Mutex { data: <locked> }
}

pub fn concurrent_with_channel() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            if i%3 == 0 {
                thread::sleep(Duration::from_millis(100));
            }

            data[i] += 1;
            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }

    // In this pattern, you wait all threads ending by using channel (waiting at `rx.recv()` point)
    // and you will get value correctly in this line.
    println!("{:?}", data);
}
