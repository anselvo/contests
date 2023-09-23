use playground::{algorithms, structures};

mod chat;

#[tokio::main]
async fn main() {
    println!("Hello Playground!");
    println!("New Vector: {:?}", structures::vector::Vec::new());
    println!("Find 3: {:?}", algorithms::search::binary_while(vec!(1, 2, 3, 4), 3).unwrap());

    chat::run("localhost", 8080).await;
}
