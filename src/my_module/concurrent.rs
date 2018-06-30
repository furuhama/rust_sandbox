use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn concurrent() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    for i in 0..10 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            // process written below is an experiment to do something which takes long time in some threads
            if data[i]%3 == 0 {
                thread::sleep(Duration::from_millis(50));
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
