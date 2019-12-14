use futures::executor::block_on;

pub fn async_sample() {
    let hello = hello_async();
    block_on(hello);
}

async fn hello_async() {
    println!("Hello async!");
}
