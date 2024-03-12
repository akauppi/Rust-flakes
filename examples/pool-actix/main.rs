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
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}
