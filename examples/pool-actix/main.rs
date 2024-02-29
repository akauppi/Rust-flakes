mod pool;
mod fish;

use std::env;
use std::time::Duration;

use pool::Pool;
use fish::Fish;

use actix::prelude::*;

use log::debug;

#[actix_rt::main]
async fn main() {
    enable_debug();
    let pool = Pool::new();

    let fut1 = pool.fish_for(Fish::Ahven, Duration::from_secs(3));
    let fut2 = pool.fish_for(Fish::Hauki, Duration::from_secs(10));

    futures_util::join!(fut1, fut2);    // nb. macro does the '.await'

    debug!("Finished.");
}

fn enable_debug() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();
}