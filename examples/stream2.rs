/*
* examples/stream2.rs
*
* References:
*   - "A Guided Tour of Streams in Rust" (blog, May'22)
*       -> https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust/#a-higherlevel-api
*/
use futures;

/*
* Usage:
*   $ cargo run --example stream2
*/
async fn a_main() {
    use futures::stream::{self, StreamExt};

    let stream = stream::iter(vec![17, 19]);
    assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);
}

fn main() {
    futures::executor::block_on(a_main())
}
