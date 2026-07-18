use futures::executor::block_on;

async fn hi() {
    println!("Hello, world!");
    hello().await;
}

async fn hello() {
    println!("Hello, world!");
}

fn main() {
    let function  = hi();
    block_on(function);
}