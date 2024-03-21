/*
* Samples of using 'futures_util::stream::Stream'
*/

use futures_util::stream::{self, Stream, StreamExt as _};
use futures_executor::block_on;

use log::{info, debug};

//R use rust_flakes::extra_stream::Parcel;

// NOTE!!!!!
//
//  The _TRICK_ was to use '.await' at the end of the '.collect()'!  The error messages definitely
//  did not push in that direction - how could they?=
//
#[cfg(works)]
async fn just_map() {
    let x = stream::iter(&[1,2,3]);

    let xx: Vec<i32> = x
        //.inspect(|x| { debug!("{:?}",x); })
        .collect() .await;

    // why??
    debug!("{:?}", xx);
}

/** disabled
async fn stream_flatmap() {
    let s1 = Box::new(&[1,2,3][..]);
    let s2 = Box::new(&[4,5,6,7][..]);
        // Rust note: both '&' and '[..]' ARE needed; otherwise the pieces won't click.

    let x: dyn Stream<Item=()> = stream::iter([s1,s2]);
        //
        // Q: What should the 'Item' be here???
        //
        // Rust compiler: "expected `dyn Stream`, found `Iter<IntoIter<Box<&[{integer}]>, 2>>`"

    let xx: dyn Stream<Item=()> = {
        x.inspect(|x| {
            debug!("{:?}",x);
        }).map(|x| {
            let z = *x;
            stream::iter(z)
        })
    };
}**/

async fn parcel_bytes() {
    let DATA: Box<&[u8]> = Box::new(&[1,2,3,0x7e,5,6,7,8,9,0x7e][..]);

    #[cfg(fails)]
    let x: dyn Stream<Item=u8> = stream::iter(DATA.to_vec());
        // Rust compiler: "expected `dyn Stream`, found `Iter<IntoIter<u8>>`"

    let x = stream::iter(DATA.to_vec()) as Stream<Item=u8>;

    let xx = x.inspect(|x| debug!("{}", x))
        .collect::<Vec<u8>>() .await;
}

fn main() -> Result<(),String> {
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("debug"))
        .init();

    info!("main");

    block_on(
        //just_map()
        //stream_flatmap()
        parcel_bytes()
    );

    Ok(())
}
