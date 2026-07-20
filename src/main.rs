use tokio;


async fn hi() {
    println!("Hello, world!");
}
#[tokio::main]
async fn main() {
    hi().await;
}