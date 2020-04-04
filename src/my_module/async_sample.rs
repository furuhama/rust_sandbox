use futures::executor::block_on;
use async_std::task;

pub fn async_sample() {
    futures_async();
    async_std_async();
}

fn futures_async() {
    let hello = hello_async();
    block_on(hello);
}

async fn hello_async() {
    println!("Hello async!");
}

fn async_std_async() {
    let mut futures = Vec::new();

    for i in 0..10 {
        let async_func = async move {
            println!("{}", i);
        };

        let handler = task::spawn(async_func);
        println!("spawned {}", i);

        futures.push(handler);
    };

    task::block_on(async {
        for h in futures {
            h.await;
        }
    });
}
