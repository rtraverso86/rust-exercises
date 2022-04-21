//! Sources and examples based on https://tokio.rs/tokio/tutorial/streams

/// Tokio `Stream`s are just like standard Rust iterators but designed to
/// work asynchronously.

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    println!("main1:");
    main1().await;
}

async fn main1() {
    let mut stream = tokio_stream::iter(&[1, 2, 3]);

    //* Currently, the Rust programming language does not support async for loops,
    //* therefore we must rely on a while-let.
    while let Some(v) = stream.next().await {
        println!("GOT = {:?}", v);
    }
}