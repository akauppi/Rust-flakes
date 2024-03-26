use log::{debug, info};

use futures_core::future::{Future, BoxFuture};
use futures_executor::block_on;

// Thanks -> https://users.rust-lang.org/t/creating-a-trait-with-functions-that-return-future/75615/4?u=asko

async fn next() -> u8 {
    0
}

/**R fn nnext() -> BoxFuture<'static, dyn Future<Output=u8>> {
    Box::pin(next())
}*/

fn main() -> Result<(),String> {
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("debug"))
        .init();

    info!("main");

    block_on(async {
        let x /*: fn() -> impl Future<u8> + Sized*/ = next;     // how to denote the type?

        //let x: Box<dyn FnMut() -> dyn Future<Output=u8>> = Box::new(next);
            // rustc: "expected `next` to be a fn item that returns `dyn Future<Output = u8>`, but it returns `impl Future<Output = u8>`"

        //let x: Box<dyn FnMut() -> impl Future<Output=u8>> = Box::new(next);
            // rustc: "`impl Trait` is not allowed in the return type of `Fn` trait bounds"

        debug!("{:?}", x() .await);
    });

    Ok(())
}
