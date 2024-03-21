/*
* Samples of using 'futures_util::stream::Stream'
*/

use futures_util::stream::{self, Stream, StreamExt};

use log::{info, debug};

fn stream_flatmap() {
    let s1 = Box::new(&[1,2,3][..]);
    let s2 = Box::new(&[4,5,6,7][..]);

    let x = stream::iter([s1,s2]);

    let xx: Vec<i32> = x.inspect(|x| {
        debug!("{:?}",x);
    }).collect();
        // Rust compiler: "expected `Vec<i32>`, found `Collect<Inspect<Iter<IntoIter<Box<&[{integer}]>, 2>>"
}

fn main() -> Result<(),String> {
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("debug"))
        .init();

    info!("main");

    Ok(())
}
